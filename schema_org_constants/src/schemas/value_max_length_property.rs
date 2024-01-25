/// <https://schema.org/valueMaxLength>
pub const VALUE_MAX_LENGTH_PROPERTY_IRI_HTTP: &str = "http://schema.org/valueMaxLength";
/// <https://schema.org/valueMaxLength>
pub const VALUE_MAX_LENGTH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/valueMaxLength";
/// <https://schema.org/valueMaxLength>
pub const VALUE_MAX_LENGTH_PROPERTY_LABEL: &str = "valueMaxLength";
pub struct ValueMaxLengthPropertyIri;
impl PartialEq<&str> for ValueMaxLengthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALUE_MAX_LENGTH_PROPERTY_IRI_HTTP
			|| *other == VALUE_MAX_LENGTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValueMaxLengthPropertyIri> for &str {
	fn eq(&self, other: &ValueMaxLengthPropertyIri) -> bool {
		*self == VALUE_MAX_LENGTH_PROPERTY_IRI_HTTP || *self == VALUE_MAX_LENGTH_PROPERTY_IRI_HTTPS
	}
}
pub struct ValueMaxLengthPropertyIriOrLabel;
impl PartialEq<&str> for ValueMaxLengthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValueMaxLengthPropertyIri || *other == VALUE_MAX_LENGTH_PROPERTY_LABEL
	}
}
impl PartialEq<ValueMaxLengthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValueMaxLengthPropertyIriOrLabel) -> bool {
		*self == ValueMaxLengthPropertyIri || *self == VALUE_MAX_LENGTH_PROPERTY_LABEL
	}
}
