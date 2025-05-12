use uuid::{NonNilUuid, Uuid};
use crate::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

#[Scalar(
    internal,
    name = "UUID",
    specified_by_url = "http://tools.ietf.org/html/rfc4122"
)]
/// A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
/// Strings within GraphQL. UUIDs are used to assign unique identifiers to
/// entities without requiring a central allocating authority.
///
/// # References
///
/// * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
/// * [RFC4122: A Universally Unique Identifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
impl ScalarType for Uuid {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(Uuid::parse_str(&s)?),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[Scalar(
    internal,
    name = "NonNilUUID",
    specified_by_url = "http://tools.ietf.org/html/rfc4122"
)]
/// A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
/// Strings within GraphQL. UUIDs are used to assign unique identifiers to
/// entities without requiring a central allocating authority.
///
/// # References
///
/// * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
/// * [RFC4122: A Universally Unique Identifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
impl ScalarType for NonNilUuid {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                let uuid = Uuid::parse_str(&s)?;
                Ok(NonNilUuid::try_from(uuid)?)
            },
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}
