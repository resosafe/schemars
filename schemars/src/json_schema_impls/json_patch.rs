use crate::{gen::SchemaGenerator, Map, Set};
use crate::schema::*;
use crate::JsonSchema;
use std::borrow::Cow;
use json_patch::{Patch, PatchOperation};

impl JsonSchema for Patch {
    fn schema_name() -> String {
        "Patch".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("json_patch::Patch")
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<PatchOperation>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}


impl JsonSchema for PatchOperation {
    fn schema_name() -> String {
        "PatchOperation".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("json_patch::PatchOperation")
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                properties: {
                    let mut map = Map::new();
                    map.insert("op".to_owned(), gen.subschema_for::<String>());
                    map.insert("path".to_owned(), gen.subschema_for::<String>());
                    map.insert("from".to_owned(), gen.subschema_for::<String>());
                    map.insert("value".to_owned(), gen.subschema_for::<serde_json::Value>());
                    map
                },
                required: {
                    let mut required = Set::new();
                    required.insert("op".to_string());
                    required.insert("path".to_string());
                    required
                },
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}