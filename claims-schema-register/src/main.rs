use schema_registry_converter::blocking::schema_registry::{post_schema, SrSettings};
use schema_registry_converter::error::SRCError;
use schema_registry_converter::schema_registry_common::{RegisteredSchema, SchemaType, SuppliedReference, SuppliedSchema};
use claims_schema::{RAW_SCHEMA_CLAIM, RAW_SCHEMA_CLAIM_STATUS, RAW_SCHEMA_INCIDENT_STATUS, SCHEMA_NAME_CLAIM, SCHEMA_NAME_CLAIM_STATUS, SCHEMA_NAME_INCIDENT_STATUS};

fn main() {
    tracing_subscriber::fmt::init();

    let schemas = vec![
        SchemaToRegister::new(SCHEMA_NAME_CLAIM_STATUS, RAW_SCHEMA_CLAIM_STATUS, vec![]),
        SchemaToRegister::new(SCHEMA_NAME_INCIDENT_STATUS, RAW_SCHEMA_INCIDENT_STATUS, vec![]),
        SchemaToRegister::new(SCHEMA_NAME_CLAIM, RAW_SCHEMA_CLAIM, vec![SCHEMA_NAME_CLAIM_STATUS, SCHEMA_NAME_INCIDENT_STATUS]),
    ];

    for s in &schemas {
        let references = {
            let mut references = vec![];
            for rs in &s.reference_subjects {
                if let Some(ref_schema) = schemas.iter().find(|s| s.subject_name == *rs) {
                    references.push(
                        SuppliedReference {
                            name: ref_schema.subject_name.into(),
                            subject: ref_schema.subject_name.into(),
                            schema: ref_schema.schema_definition.into(),
                            references: vec![],
                        }
                    )
                }
            }

            references
        };
        register_schema(&"http://localhost:58003", s.subject_name, s.schema_definition, references);
    }
}

struct SchemaToRegister<'a> {
    subject_name: &'a str,
    schema_definition: &'a str,
    reference_subjects: Vec<&'a str>,
}

impl<'a> SchemaToRegister<'a> {
    fn new(name: &'a str, schema: &'a str, references: Vec<&'a str>) -> SchemaToRegister<'a> {
        Self {
            subject_name: name,
            schema_definition: schema,
            reference_subjects: references,
        }
    }
}

fn register_schema(schema_registry_url: &str, subject_name: &str, schema_definition: &str, references: Vec<SuppliedReference>) {
    let schema = SuppliedSchema {
        name: Some(subject_name.to_owned()),
        schema_type: SchemaType::Protobuf,
        schema: schema_definition.to_owned(),
        references
    };

    let result = register_schema_as_subject(schema_registry_url, subject_name, schema);

    match result {
        Ok(registered_schema) => tracing::info!(
            "Registered schema \"{}\" with id: {}",
            subject_name, registered_schema.id
        ),
        Err(e) => tracing::error!("Failed to register schema \"{}\": \n{}", subject_name, e),
    }
}


fn register_schema_as_subject(
    registry_url: &str,
    subject: &str,
    schema: SuppliedSchema,
) -> Result<RegisteredSchema, SRCError> {
    let sr_settings = SrSettings::new_builder(registry_url.to_owned())
        .build()
        .expect("Initialization of schema registry configuration failed");

    post_schema(&sr_settings, subject.to_owned(), schema)
}