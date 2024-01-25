/// <https://schema.org/value>
pub const VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/value";
/// <https://schema.org/value>
pub const VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/value";
/// <https://schema.org/value>
pub const VALUE_PROPERTY_LABEL: &str = "value";
pub struct ValuePropertyIri;
impl PartialEq<&str> for ValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALUE_PROPERTY_IRI_HTTP || *other == VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValuePropertyIri> for &str {
	fn eq(&self, other: &ValuePropertyIri) -> bool {
		*self == VALUE_PROPERTY_IRI_HTTP || *self == VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct ValuePropertyIriOrLabel;
impl PartialEq<&str> for ValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValuePropertyIri || *other == VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<ValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValuePropertyIriOrLabel) -> bool {
		*self == ValuePropertyIri || *self == VALUE_PROPERTY_LABEL
	}
}
