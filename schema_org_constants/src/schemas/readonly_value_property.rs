/// <https://schema.org/readonlyValue>
pub const READONLY_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/readonlyValue";
/// <https://schema.org/readonlyValue>
pub const READONLY_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/readonlyValue";
/// <https://schema.org/readonlyValue>
pub const READONLY_VALUE_PROPERTY_LABEL: &str = "readonlyValue";
pub struct ReadonlyValuePropertyIri;
impl PartialEq<&str> for ReadonlyValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == READONLY_VALUE_PROPERTY_IRI_HTTP || *other == READONLY_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReadonlyValuePropertyIri> for &str {
	fn eq(&self, other: &ReadonlyValuePropertyIri) -> bool {
		*self == READONLY_VALUE_PROPERTY_IRI_HTTP || *self == READONLY_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct ReadonlyValuePropertyIriOrLabel;
impl PartialEq<&str> for ReadonlyValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReadonlyValuePropertyIri || *other == READONLY_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<ReadonlyValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReadonlyValuePropertyIriOrLabel) -> bool {
		*self == ReadonlyValuePropertyIri || *self == READONLY_VALUE_PROPERTY_LABEL
	}
}
