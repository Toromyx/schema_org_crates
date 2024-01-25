/// <https://schema.org/valueRequired>
pub const VALUE_REQUIRED_PROPERTY_IRI_HTTP: &str = "http://schema.org/valueRequired";
/// <https://schema.org/valueRequired>
pub const VALUE_REQUIRED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/valueRequired";
/// <https://schema.org/valueRequired>
pub const VALUE_REQUIRED_PROPERTY_LABEL: &str = "valueRequired";
pub struct ValueRequiredPropertyIri;
impl PartialEq<&str> for ValueRequiredPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALUE_REQUIRED_PROPERTY_IRI_HTTP || *other == VALUE_REQUIRED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValueRequiredPropertyIri> for &str {
	fn eq(&self, other: &ValueRequiredPropertyIri) -> bool {
		*self == VALUE_REQUIRED_PROPERTY_IRI_HTTP || *self == VALUE_REQUIRED_PROPERTY_IRI_HTTPS
	}
}
pub struct ValueRequiredPropertyIriOrLabel;
impl PartialEq<&str> for ValueRequiredPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValueRequiredPropertyIri || *other == VALUE_REQUIRED_PROPERTY_LABEL
	}
}
impl PartialEq<ValueRequiredPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValueRequiredPropertyIriOrLabel) -> bool {
		*self == ValueRequiredPropertyIri || *self == VALUE_REQUIRED_PROPERTY_LABEL
	}
}
