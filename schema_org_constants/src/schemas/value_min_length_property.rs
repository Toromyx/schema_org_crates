/// <https://schema.org/valueMinLength>
pub const VALUE_MIN_LENGTH_PROPERTY_IRI_HTTP: &str = "http://schema.org/valueMinLength";
/// <https://schema.org/valueMinLength>
pub const VALUE_MIN_LENGTH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/valueMinLength";
/// <https://schema.org/valueMinLength>
pub const VALUE_MIN_LENGTH_PROPERTY_LABEL: &str = "valueMinLength";
pub struct ValueMinLengthPropertyIri;
impl PartialEq<&str> for ValueMinLengthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALUE_MIN_LENGTH_PROPERTY_IRI_HTTP
			|| *other == VALUE_MIN_LENGTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValueMinLengthPropertyIri> for &str {
	fn eq(&self, other: &ValueMinLengthPropertyIri) -> bool {
		*self == VALUE_MIN_LENGTH_PROPERTY_IRI_HTTP || *self == VALUE_MIN_LENGTH_PROPERTY_IRI_HTTPS
	}
}
pub struct ValueMinLengthPropertyIriOrLabel;
impl PartialEq<&str> for ValueMinLengthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValueMinLengthPropertyIri || *other == VALUE_MIN_LENGTH_PROPERTY_LABEL
	}
}
impl PartialEq<ValueMinLengthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValueMinLengthPropertyIriOrLabel) -> bool {
		*self == ValueMinLengthPropertyIri || *self == VALUE_MIN_LENGTH_PROPERTY_LABEL
	}
}
