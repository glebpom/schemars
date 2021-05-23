use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use language_tags::LanguageTag;

impl JsonSchema for LanguageTag {
    no_ref_schema!();

    fn schema_name() -> String {
        "LanguageTag".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}
