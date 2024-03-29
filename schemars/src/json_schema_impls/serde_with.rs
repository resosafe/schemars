use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use serde_json::json;
use std::borrow::Cow;
use std::ops::{Bound, Range, RangeInclusive};
use serde_with::{As, DisplayFromStr};

impl<T: JsonSchema> JsonSchema for As<T> {
    no_ref_schema!();

    fn schema_name() -> String {
        format!("serde_with::As_{}", T::schema_name())
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Owned(format!("serde_with::As<{}>", T::schema_id()))
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
       T::json_schema(gen)
    }

    fn _schemars_private_non_optional_json_schema(gen: &mut SchemaGenerator) -> Schema {
        T::_schemars_private_non_optional_json_schema(gen)
    }

    fn _schemars_private_is_option() -> bool {
        true
    }
}

// impl  JsonSchema for As<Option<DisplayFromStr>> {

//     fn schema_name() -> String {
//         "serde_with::As_DisplayFromStr".to_owned()
//     }

//     fn schema_id() -> Cow<'static, str> {
//         Cow::Owned("serde_with::As<DisplayFromStr>".to_owned())
//     }

//     fn json_schema(gen: &mut SchemaGenerator) -> Schema {
//         SchemaObject {
//             instance_type: Some(InstanceType::String.into()),
//             ..Default::default()
//         }
//         .into()
//     }

// }

impl<T: JsonSchema> JsonSchema for serde_with::VecSkipError<T> {
    no_ref_schema!();

    fn schema_name() -> String {
        "serde_with::VecSkipError".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Owned("serde_with::VecSkipError".to_owned())
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<T>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }


}

impl JsonSchema for serde_with::Same {
    no_ref_schema!();

    fn schema_name() -> String {
        "serde_with::Same".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Owned("serde_with::Same".to_owned())
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }


}

impl JsonSchema for DisplayFromStr {
    no_ref_schema!();

    fn schema_name() -> String {
        "serde_with::DisplayFromStr".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Owned("serde_with::DisplayFromStr".to_owned())
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }

}

// impl<T: JsonSchema> JsonSchema for DisplayFromStr<T> {
//     no_ref_schema!();

//     fn schema_name() -> String {
//         format!("serde_with::DisplayFromStr_{}", T::schema_name())
//     }

//     fn schema_id() -> Cow<'static, str> {
//         Cow::Owned(format!("serde_with::DisplayFromStr<{}>", T::schema_id()))
//     }

//     fn json_schema(gen: &mut SchemaGenerator) -> Schema {
//        T::json_schema(gen)
//     }
// }