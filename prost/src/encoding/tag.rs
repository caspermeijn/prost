use varint::Varint;

use super::*;

pub struct FieldNumber {
    pub value: u32
}

impl FieldNumber {
    pub fn new(field_number: u32) -> Self {
        debug_assert!((MIN_TAG..=MAX_TAG).contains(&field_number));
        Self { value: field_number }
    }
}

impl TryFrom<u64> for FieldNumber {
    type Error = DecodeError;
    
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < MIN_TAG as u64 || value > MAX_TAG as u64 {
            Err(DecodeError::new(format!("invalid field number value: {}", value)))
        } else {
            Ok(Self {
                value: value as u32
            })
        }
    }
}

pub struct Tag {
    pub field_number: FieldNumber,
    pub wire_type: WireType,
}

impl TryFrom<u64> for Tag {
    type Error = DecodeError;
    
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let wire_type = WireType::try_from(value & 0x07)?;
        let field_number = FieldNumber::try_from(value >> 3)?;
        Ok(Self {
            field_number, wire_type
        })
    }
}

impl ProtobufEncoding for Tag {
    /// Encodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline]
    fn encode(self, buf: &mut impl BufMut) {
        let tag = (self.field_number.value << 3) | self.wire_type as u32;
        Varint::from(tag).encode(buf)
    }

    /// Returns the width of an encoded Protobuf field key with the given tag.
/// The returned width will be between 1 and 5 bytes (inclusive).
#[inline]
    fn encoded_len(self) -> usize {
        Varint::from(self.field_number.value << 3).encoded_len()
    }

    /// Decodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline(always)]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let tag = Varint::decode(buf)?.value;
        Self::try_from(tag)
    }
}
