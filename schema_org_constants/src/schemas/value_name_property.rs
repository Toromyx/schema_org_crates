/// <https://schema.org/valueName>
pub const VALUE_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/valueName";
/// <https://schema.org/valueName>
pub const VALUE_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/valueName";
/// <https://schema.org/valueName>
pub const VALUE_NAME_PROPERTY_LABEL: &str = "valueName";
pub struct ValueNamePropertyIri;
impl PartialEq<&str> for ValueNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALUE_NAME_PROPERTY_IRI_HTTP || *other == VALUE_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValueNamePropertyIri> for &str {
	fn eq(&self, other: &ValueNamePropertyIri) -> bool {
		*self == VALUE_NAME_PROPERTY_IRI_HTTP || *self == VALUE_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct ValueNamePropertyIriOrLabel;
impl PartialEq<&str> for ValueNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValueNamePropertyIri || *other == VALUE_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<ValueNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValueNamePropertyIriOrLabel) -> bool {
		*self == ValueNamePropertyIri || *self == VALUE_NAME_PROPERTY_LABEL
	}
}
