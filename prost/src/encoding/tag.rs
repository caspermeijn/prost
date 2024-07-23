use alloc::format;
use core::u32;
use core::usize;

use ::bytes::{Buf, BufMut};

use crate::DecodeError;

use super::FieldNumber;
use super::ProtobufDecode;
use super::ProtobufEncode;
use super::Varint;
use super::WireType;

/// A tag is the combination of field number and wire type in protobuf encoding
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Tag {
    pub field_number: FieldNumber,
    pub wire_type: WireType,
}

impl Tag {
    pub fn new(field_number: FieldNumber, wire_type: WireType) -> Self {
        Tag {
            field_number,
            wire_type,
        }
    }
}

impl ProtobufEncode for Tag {
    /// Encodes a Protobuf field tag, which consists of a wire type designator and
    /// the field number into LEB128 format, and writes it to the buffer.
    ///
    /// The current position of `buf` is advanced.
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `buf`. See [`Self::encoded_len()`] for the required length
    fn encode(&self, buf: &mut impl BufMut) {
        let tag: u32 = self.field_number.into();
        let key = (tag << 3) | self.wire_type as u32;
        Varint::from(u64::from(key)).encode(buf)
    }

    /// Returns the number of bytes required to encode this Protobuf field tag in LEB128 variable length format.
    /// The returned width will be between 1 and 5 bytes (inclusive).
    fn encoded_len(&self) -> usize {
        let tag: u32 = self.field_number.into();
        let key = u64::from(tag << 3);
        Varint::from(key).encoded_len()
    }
}

impl ProtobufDecode for Tag {
    /// Decodes a Protobuf field key, which consists of a wire type designator and
    /// the field tag.
    ///
    /// The current position of `buf` is advanced.
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let key: u64 = Varint::decode(buf)?.into();
        if key > u64::from(u32::MAX) {
            return Err(DecodeError::new(format!("invalid key value: {}", key)));
        }
        let wire_type = WireType::try_from(key & 0x07)?;
        let field_number = FieldNumber::try_from(key as u32 >> 3)?;

        Ok(Tag {
            field_number,
            wire_type,
        })
    }
}

// impl From<(FieldNumber, WireType)> for Tag {
//     fn from(value: (FieldNumber, WireType)) -> Self {
//         Self {
//             field_number: value.0,
//             wire_type: value.1,
//         }
//     }
// }

// impl From<Tag> for (FieldNumber, WireType) {
//     fn from(value: Tag) -> Self {
//         (value.field_number, value.wire_type)
//     }
// }
