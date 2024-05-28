//! Protobuf encoding and decoding errors.

use core::fmt;
use crate::encoding::WireType;

/// A Protobuf message decoding error.
///
/// `DecodeError` indicates that the input buffer does not contain a valid
/// Protobuf message. The error details should be considered 'best effort': in
/// general it is not possible to exactly pinpoint why data is malformed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// Length delimiter exceeds maximum usize value
    LengthDelimiterTooLarge,
    /// Invalid varint
    InvalidVarint,
    #[cfg(not(feature = "no-recursion-limit"))]
    /// Recursion limit reached
    RecursionLimitReached,
    /// Invalid wire type value
    InvalidWireType {value: u64},
    /// Invalid key value
    InvalidKey {value: u64},
    /// Invalid tag value: 0
    InvalidTag,
    /// Invalid wire type
    UnexpectedWireType { actual: WireType, expected: WireType },
    /// Buffer underflow
    BufferUnderflow,
    /// Delimited length exceeded
    DelimitedLengthExceeded,
    /// Unexpected end group tag
    UnexpectedEndGroupTag,
    /// Invalid string value: data is not UTF-8 encoded
    InvalidString,

}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to decode Protobuf message: ")?;
        match self {
            DecodeError::LengthDelimiterTooLarge => write!(f, "Length delimiter exceeds maximum usize value"),
            DecodeError::InvalidVarint => write!(f, "Invalid varint"),
            DecodeError::RecursionLimitReached => write!(f, "recursion limit reached"),
            DecodeError::InvalidWireType { value } => write!(f, "invalid wire type value: {value}"),
            DecodeError::InvalidKey { value } => write!(f, "invalid key value: {value}"),
            DecodeError::InvalidTag => write!(f, "invalid tag value: 0"),
            DecodeError::UnexpectedWireType { actual, expected } => write!(f, "invalid wire type: {actual} (expected {expected})"),
            DecodeError::BufferUnderflow => write!(f, "buffer underflow"),
            DecodeError::DelimitedLengthExceeded => write!(f, "delimited length exceeded"),
            DecodeError::UnexpectedEndGroupTag => write!(f, "unexpected end group tag"),
            DecodeError::InvalidString => write!(f, "invalid string value: data is not UTF-8 encoded"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

#[cfg(feature = "std")]
impl From<DecodeError> for std::io::Error {
    fn from(error: DecodeError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, error)
    }
}

/// A Protobuf message encoding error.
///
/// `EncodeError` always indicates that a message failed to encode because the
/// provided buffer had insufficient capacity. Message encoding is otherwise
/// infallible.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EncodeError {
    required: usize,
    remaining: usize,
}

impl EncodeError {
    /// Creates a new `EncodeError`.
    pub(crate) fn new(required: usize, remaining: usize) -> EncodeError {
        EncodeError {
            required,
            remaining,
        }
    }

    /// Returns the required buffer capacity to encode the message.
    pub fn required_capacity(&self) -> usize {
        self.required
    }

    /// Returns the remaining length in the provided buffer at the time of encoding.
    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to encode Protobuf message; insufficient buffer capacity (required: {}, remaining: {})",
            self.required, self.remaining
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EncodeError {}

#[cfg(feature = "std")]
impl From<EncodeError> for std::io::Error {
    fn from(error: EncodeError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, error)
    }
}

/// An error indicating that an unknown enumeration value was encountered.
///
/// The Protobuf spec mandates that enumeration value sets are ‘open’, so this
/// error's value represents an integer value unrecognized by the
/// presently used enum definition.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UnknownEnumValue(pub i32);

impl fmt::Display for UnknownEnumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown enumeration value {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnknownEnumValue {}
