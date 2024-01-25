/// <https://schema.org/defaultValue>
pub const DEFAULT_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/defaultValue";
/// <https://schema.org/defaultValue>
pub const DEFAULT_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/defaultValue";
/// <https://schema.org/defaultValue>
pub const DEFAULT_VALUE_PROPERTY_LABEL: &str = "defaultValue";
pub struct DefaultValuePropertyIri;
impl PartialEq<&str> for DefaultValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEFAULT_VALUE_PROPERTY_IRI_HTTP || *other == DEFAULT_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DefaultValuePropertyIri> for &str {
	fn eq(&self, other: &DefaultValuePropertyIri) -> bool {
		*self == DEFAULT_VALUE_PROPERTY_IRI_HTTP || *self == DEFAULT_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct DefaultValuePropertyIriOrLabel;
impl PartialEq<&str> for DefaultValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DefaultValuePropertyIri || *other == DEFAULT_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<DefaultValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DefaultValuePropertyIriOrLabel) -> bool {
		*self == DefaultValuePropertyIri || *self == DEFAULT_VALUE_PROPERTY_LABEL
	}
}
