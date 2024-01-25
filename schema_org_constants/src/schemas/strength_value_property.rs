/// <https://schema.org/strengthValue>
pub const STRENGTH_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/strengthValue";
/// <https://schema.org/strengthValue>
pub const STRENGTH_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/strengthValue";
/// <https://schema.org/strengthValue>
pub const STRENGTH_VALUE_PROPERTY_LABEL: &str = "strengthValue";
pub struct StrengthValuePropertyIri;
impl PartialEq<&str> for StrengthValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STRENGTH_VALUE_PROPERTY_IRI_HTTP || *other == STRENGTH_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StrengthValuePropertyIri> for &str {
	fn eq(&self, other: &StrengthValuePropertyIri) -> bool {
		*self == STRENGTH_VALUE_PROPERTY_IRI_HTTP || *self == STRENGTH_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct StrengthValuePropertyIriOrLabel;
impl PartialEq<&str> for StrengthValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StrengthValuePropertyIri || *other == STRENGTH_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<StrengthValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &StrengthValuePropertyIriOrLabel) -> bool {
		*self == StrengthValuePropertyIri || *self == STRENGTH_VALUE_PROPERTY_LABEL
	}
}
