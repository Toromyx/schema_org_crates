/// <https://schema.org/additionalVariable>
pub const ADDITIONAL_VARIABLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/additionalVariable";
/// <https://schema.org/additionalVariable>
pub const ADDITIONAL_VARIABLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/additionalVariable";
/// <https://schema.org/additionalVariable>
pub const ADDITIONAL_VARIABLE_PROPERTY_LABEL: &str = "additionalVariable";
pub struct AdditionalVariablePropertyIri;
impl PartialEq<&str> for AdditionalVariablePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDITIONAL_VARIABLE_PROPERTY_IRI_HTTP
			|| *other == ADDITIONAL_VARIABLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AdditionalVariablePropertyIri> for &str {
	fn eq(&self, other: &AdditionalVariablePropertyIri) -> bool {
		*self == ADDITIONAL_VARIABLE_PROPERTY_IRI_HTTP
			|| *self == ADDITIONAL_VARIABLE_PROPERTY_IRI_HTTPS
	}
}
pub struct AdditionalVariablePropertyIriOrLabel;
impl PartialEq<&str> for AdditionalVariablePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdditionalVariablePropertyIri || *other == ADDITIONAL_VARIABLE_PROPERTY_LABEL
	}
}
impl PartialEq<AdditionalVariablePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AdditionalVariablePropertyIriOrLabel) -> bool {
		*self == AdditionalVariablePropertyIri || *self == ADDITIONAL_VARIABLE_PROPERTY_LABEL
	}
}
