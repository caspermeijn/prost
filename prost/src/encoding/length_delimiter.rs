use varint::Varint;

use super::*;

pub struct LengthDelimiter {
    pub value: usize,
}

impl TryFrom<u64> for LengthDelimiter {
    type Error = DecodeError;
    
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > usize::MAX as u64 {
            Err(DecodeError::new(
                "length delimiter exceeds maximum usize value",
            ))
        } else {
            Ok(Self {
                value: value as usize
            })
        }
    }
}

impl ProtobufEncoding for LengthDelimiter {
    fn encode(self, buf: &mut impl BufMut) {
        Varint::from(self.value).encode(buf)
    }

    fn encoded_len(self) -> usize {
        Varint::from(self.value).encoded_len()
    }

    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let length = Varint::decode(buf)?.value;
        Self::try_from(length)
    }
}
