pub use crate::error::{DecodeError, EncodeError, UnknownEnumValue};
pub use crate::message::Message;
pub use crate::name::Name;

use bytes::{Buf, BufMut};

use crate::encoding::varint::Varint;
use crate::encoding::ProtobufEncode;
use crate::encoding::ProtobufDecode;

/// An length value encoded as LEB128 variable length format.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LengthDelimiter {
    value: usize,
}

impl From<usize> for LengthDelimiter {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl From<LengthDelimiter> for usize {
    fn from(value: LengthDelimiter) -> Self {
        value.value
    }
}

impl ProtobufEncode for LengthDelimiter {
    /// Encodes an length delimiter into LEB128 variable length format, and writes it to the buffer.
    ///
    /// The current position of `buf` is advanced.
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `buf`. See [`Self::encoded_len()`] for the required length
    fn encode(&self, buf: &mut impl BufMut) {
        Varint::from(self.value as u64).encode(buf)
    }

    fn encoded_len(&self) -> usize {
        Varint::from(self.value as u64).encoded_len()
    }
}

impl ProtobufDecode for LengthDelimiter {
    /// Decode an length delimiter from LEB128 variable length format.
    /// If the value doesn't fit into a usize this result in an [`DecodeError`].
    ///
    /// The current position of `buf` is advanced.
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let length: u64 = Varint::decode(buf)?.into();
        if length > usize::MAX as u64 {
            return Err(DecodeError::new(
                "length delimiter exceeds maximum usize value",
            ));
        }
        Ok(Self::from(length as usize))
    }
}

/// Encodes a length delimiter to the buffer.
///
/// See [Message.encode_length_delimited] for more info.
///
/// An error will be returned if the buffer does not have sufficient capacity to encode the
/// delimiter.
pub fn encode_length_delimiter(length: usize, buf: &mut impl BufMut) -> Result<(), EncodeError> {
    let delimiter = LengthDelimiter::from(length);
    let required = delimiter.encoded_len();
    let remaining = buf.remaining_mut();
    if required > remaining {
        return Err(EncodeError::new(required, remaining));
    }
    delimiter.encode(buf);
    Ok(())
}

/// Returns the encoded length of a length delimiter.
///
/// Applications may use this method to ensure sufficient buffer capacity before calling
/// `encode_length_delimiter`. The returned size will be between 1 and 10, inclusive.
pub fn length_delimiter_len(length: usize) -> usize {
    LengthDelimiter::from(length).encoded_len()
}

/// Decodes a length delimiter from the buffer.
///
/// This method allows the length delimiter to be decoded independently of the message, when the
/// message is encoded with [Message.encode_length_delimited].
///
/// An error may be returned in two cases:
///
///  * If the supplied buffer contains fewer than 10 bytes, then an error indicates that more
///    input is required to decode the full delimiter.
///  * If the supplied buffer contains more than 10 bytes, then the buffer contains an invalid
///    delimiter, and typically the buffer should be considered corrupt.
pub fn decode_length_delimiter(mut buf: impl Buf) -> Result<usize, DecodeError> {
    LengthDelimiter::decode(&mut buf).map(usize::from)
}
