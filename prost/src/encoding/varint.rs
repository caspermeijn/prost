use core::cmp::min;

use bytes::{Buf, BufMut};

use crate::DecodeError;

use super::ProtobufEncoding;

pub struct Varint {
    pub value: u64,
}

impl From<u8> for Varint {
    fn from(value: u8) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<u32> for Varint {
    fn from(value: u32) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<u64> for Varint {
    fn from(value: u64) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<usize> for Varint {
    fn from(value: usize) -> Self {
        Self {
            value: value as u64,
        }
    }
}

impl ProtobufEncoding for Varint {
    /// Encodes an integer value into LEB128 variable length format, and writes it to the buffer.
    /// The buffer must have enough remaining space (maximum 10 bytes).
    fn encode(self, buf: &mut impl BufMut) {
        let mut value = self.value;
        // Varints are never more than 10 bytes
        for _ in 0..10 {
            if value < 0x80 {
                buf.put_u8(value as u8);
                break;
            } else {
                buf.put_u8(((value & 0x7F) | 0x80) as u8);
                value >>= 7;
            }
        }
    }

    /// Returns the encoded length of the value in LEB128 variable length format.
    /// The returned value will be between 1 and 10, inclusive.
    fn encoded_len(self) -> usize {
        // Based on [VarintSize64][1].
        // [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.h#L1301-L1309
        ((((self.value | 1).leading_zeros() ^ 63) * 9 + 73) / 64) as usize
    }

    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let bytes = buf.chunk();
        let len = bytes.len();
        if len == 0 {
            return Err(DecodeError::new("invalid varint"));
        }

        let byte = bytes[0];
        if byte < 0x80 {
            buf.advance(1);
            Ok(byte.into())
        } else if len > 10 || bytes[len - 1] < 0x80 {
            let (value, advance) = decode_varint_slice(bytes)?;
            buf.advance(advance);
            Ok(value.into())
        } else {
            decode_varint_slow(buf).map(Into::into)
        }
    }
}

/// Decodes a LEB128-encoded variable length integer from the slice, returning the value and the
/// number of bytes read.
///
/// Based loosely on [`ReadVarint64FromArray`][1] with a varint overflow check from
/// [`ConsumeVarint`][2].
///
/// ## Safety
///
/// The caller must ensure that `bytes` is non-empty and either `bytes.len() >= 10` or the last
/// element in bytes is < `0x80`.
///
/// [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.cc#L365-L406
/// [2]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
#[inline]
fn decode_varint_slice(bytes: &[u8]) -> Result<(u64, usize), DecodeError> {
    // Fully unrolled varint decoding loop. Splitting into 32-bit pieces gives better performance.

    // Use assertions to ensure memory safety, but it should always be optimized after inline.
    assert!(!bytes.is_empty());
    assert!(bytes.len() > 10 || bytes[bytes.len() - 1] < 0x80);

    let mut b: u8 = unsafe { *bytes.get_unchecked(0) };
    let mut part0: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((u64::from(part0), 1));
    };
    part0 -= 0x80;
    b = unsafe { *bytes.get_unchecked(1) };
    part0 += u32::from(b) << 7;
    if b < 0x80 {
        return Ok((u64::from(part0), 2));
    };
    part0 -= 0x80 << 7;
    b = unsafe { *bytes.get_unchecked(2) };
    part0 += u32::from(b) << 14;
    if b < 0x80 {
        return Ok((u64::from(part0), 3));
    };
    part0 -= 0x80 << 14;
    b = unsafe { *bytes.get_unchecked(3) };
    part0 += u32::from(b) << 21;
    if b < 0x80 {
        return Ok((u64::from(part0), 4));
    };
    part0 -= 0x80 << 21;
    let value = u64::from(part0);

    b = unsafe { *bytes.get_unchecked(4) };
    let mut part1: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 5));
    };
    part1 -= 0x80;
    b = unsafe { *bytes.get_unchecked(5) };
    part1 += u32::from(b) << 7;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 6));
    };
    part1 -= 0x80 << 7;
    b = unsafe { *bytes.get_unchecked(6) };
    part1 += u32::from(b) << 14;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 7));
    };
    part1 -= 0x80 << 14;
    b = unsafe { *bytes.get_unchecked(7) };
    part1 += u32::from(b) << 21;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 8));
    };
    part1 -= 0x80 << 21;
    let value = value + ((u64::from(part1)) << 28);

    b = unsafe { *bytes.get_unchecked(8) };
    let mut part2: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((value + (u64::from(part2) << 56), 9));
    };
    part2 -= 0x80;
    b = unsafe { *bytes.get_unchecked(9) };
    part2 += u32::from(b) << 7;
    // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
    // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
    if b < 0x02 {
        return Ok((value + (u64::from(part2) << 56), 10));
    };

    // We have overrun the maximum size of a varint (10 bytes) or the final byte caused an overflow.
    // Assume the data is corrupt.
    Err(DecodeError::new("invalid varint"))
}

/// Decodes a LEB128-encoded variable length integer from the buffer, advancing the buffer as
/// necessary.
///
/// Contains a varint overflow check from [`ConsumeVarint`][1].
///
/// [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
#[inline(never)]
#[cold]
fn decode_varint_slow(buf: &mut impl Buf) -> Result<u64, DecodeError> {
    let mut value = 0;
    for count in 0..min(10, buf.remaining()) {
        let byte = buf.get_u8();
        value |= u64::from(byte & 0x7F) << (count * 7);
        if byte <= 0x7F {
            // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
            // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
            if count == 9 && byte >= 0x02 {
                return Err(DecodeError::new("invalid varint"));
            } else {
                return Ok(value);
            }
        }
    }

    Err(DecodeError::new("invalid varint"))
}
