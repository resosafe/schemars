use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use serde_json::{Map, Number, Value};
use std::borrow::Cow;
use std::collections::BTreeMap;

impl JsonSchema for Value {
    no_ref_schema!();

    fn schema_name() -> String {
        "Json Value".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("JsonValue")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(SingleOrVec::Vec(
            vec![
                InstanceType::Null,
                InstanceType::Boolean,
                InstanceType::Number,
                InstanceType::String,
                InstanceType::Array,
                InstanceType::Object,
            ]
            .into_iter()
            .map(InstanceType::from)
            .collect(),
        )),
            ..Default::default()
        }.into()
    }
}

forward_impl!(Map<String, Value> => BTreeMap<String, Value>);

impl JsonSchema for Number {
    no_ref_schema!();

    fn schema_name() -> String {
        "Number".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("Number")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Number.into()),
            ..Default::default()
        }
        .into()
    }
}

