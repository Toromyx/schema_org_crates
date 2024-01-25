/// <https://schema.org/schemaVersion>
pub const SCHEMA_VERSION_PROPERTY_IRI_HTTP: &str = "http://schema.org/schemaVersion";
/// <https://schema.org/schemaVersion>
pub const SCHEMA_VERSION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/schemaVersion";
/// <https://schema.org/schemaVersion>
pub const SCHEMA_VERSION_PROPERTY_LABEL: &str = "schemaVersion";
pub struct SchemaVersionPropertyIri;
impl PartialEq<&str> for SchemaVersionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHEMA_VERSION_PROPERTY_IRI_HTTP || *other == SCHEMA_VERSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SchemaVersionPropertyIri> for &str {
	fn eq(&self, other: &SchemaVersionPropertyIri) -> bool {
		*self == SCHEMA_VERSION_PROPERTY_IRI_HTTP || *self == SCHEMA_VERSION_PROPERTY_IRI_HTTPS
	}
}
pub struct SchemaVersionPropertyIriOrLabel;
impl PartialEq<&str> for SchemaVersionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SchemaVersionPropertyIri || *other == SCHEMA_VERSION_PROPERTY_LABEL
	}
}
impl PartialEq<SchemaVersionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SchemaVersionPropertyIriOrLabel) -> bool {
		*self == SchemaVersionPropertyIri || *self == SCHEMA_VERSION_PROPERTY_LABEL
	}
}
