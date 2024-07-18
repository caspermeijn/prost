use core::fmt::Display;

use alloc::format;

use crate::DecodeError;

const MIN_VALUE: u32 = 1;
const MAX_VALUE: u32 = (1 << 29) - 1;

/// Represents a field number within a protobuf message.
///
/// It can only be constructed within the valid range of field numbers and
/// therefore it can always be encoded.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FieldNumber {
    value: u32,
}

impl FieldNumber {
    pub const MIN: FieldNumber = FieldNumber::new(MIN_VALUE);
    pub const MAX: FieldNumber = FieldNumber::new(MAX_VALUE);

    /// Create a instance of FieldNumber
    ///
    /// # Panics
    ///
    /// This function panics if `field_number` is out of the valid range. See [`TryFrom<u32>`] for a fallible conversion
    pub const fn new(field_number: u32) -> Self {
        if field_number < MIN_VALUE || field_number > MAX_VALUE {
            panic!("field_number is out of range");
        }
        Self {
            value: field_number,
        }
    }

    pub const fn into_inner(self) -> u32 {
        self.value
    }
}

impl TryFrom<u32> for FieldNumber {
    type Error = DecodeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (MIN_VALUE..=MAX_VALUE).contains(&value) {
            Ok(Self { value })
        } else {
            Err(DecodeError::new(format!("invalid field number: {value}")))
        }
    }
}

impl From<FieldNumber> for u32 {
    fn from(value: FieldNumber) -> Self {
        value.value
    }
}

impl Display for FieldNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
}
