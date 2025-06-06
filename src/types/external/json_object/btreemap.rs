use std::{borrow::Cow, collections::BTreeMap, fmt::Display, str::FromStr};

use async_graphql_parser::{Positioned, types::Field};
use async_graphql_value::{from_value, to_value};
use indexmap::IndexMap;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    ContextSelectionSet, InputType, InputValueError, InputValueResult, Name, OutputType,
    ServerResult, Value,
    registry::{MetaType, MetaTypeId, Registry},
};

impl<K, V> InputType for BTreeMap<K, V>
where
    K: ToString + FromStr + Ord + Send + Sync,
    K::Err: Display,
    V: Serialize + DeserializeOwned + Send + Sync,
{
    type RawValueType = Self;

    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("JSONObject")
    }

    fn create_type_info(registry: &mut Registry) -> String {
        registry.create_input_type::<Self, _>(MetaTypeId::Scalar, |_| MetaType::Scalar {
            name: <Self as InputType>::type_name().to_string(),
            description: Some("A scalar that can represent any JSON Object value.".to_string()),
            is_valid: None,
            visible: None,
            inaccessible: false,
            tags: Default::default(),
            specified_by_url: None,
            directive_invocations: Default::default(),
        })
    }

    fn parse(value: Option<Value>) -> InputValueResult<Self> {
        let value = value.unwrap_or_default();
        match value {
            Value::Object(map) => map
                .into_iter()
                .map(|(name, value)| {
                    Ok((
                        K::from_str(&name).map_err(|err| {
                            InputValueError::<Self>::custom(format!("object key: {}", err))
                        })?,
                        from_value(value).map_err(|err| format!("object value: {}", err))?,
                    ))
                })
                .collect::<Result<_, _>>()
                .map_err(InputValueError::propagate),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        let mut map = IndexMap::new();
        for (name, value) in self {
            map.insert(
                Name::new(name.to_string()),
                to_value(value).unwrap_or_default(),
            );
        }
        Value::Object(map)
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }
}

#[cfg_attr(feature = "boxed-trait", async_trait::async_trait)]
impl<K, V> OutputType for BTreeMap<K, V>
where
    K: ToString + Ord + Send + Sync,
    V: Serialize + Send + Sync,
{
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("JSONObject")
    }

    fn create_type_info(registry: &mut Registry) -> String {
        registry.create_output_type::<Self, _>(MetaTypeId::Scalar, |_| MetaType::Scalar {
            name: <Self as OutputType>::type_name().to_string(),
            description: Some("A scalar that can represent any JSON Object value.".to_string()),
            is_valid: None,
            visible: None,
            inaccessible: false,
            tags: Default::default(),
            specified_by_url: None,
            directive_invocations: Default::default(),
        })
    }

    async fn resolve(
        &self,
        _ctx: &ContextSelectionSet<'_>,
        _field: &Positioned<Field>,
    ) -> ServerResult<Value> {
        let mut map = IndexMap::new();
        for (name, value) in self {
            map.insert(
                Name::new(name.to_string()),
                to_value(value).unwrap_or_default(),
            );
        }
        Ok(Value::Object(map))
    }
}
