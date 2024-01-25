/// <https://schema.org/Flexibility>
pub const FLEXIBILITY_IRI_HTTP: &str = "http://schema.org/Flexibility";
/// <https://schema.org/Flexibility>
pub const FLEXIBILITY_IRI_HTTPS: &str = "https://schema.org/Flexibility";
/// <https://schema.org/Flexibility>
pub const FLEXIBILITY_LABEL: &str = "Flexibility";
pub struct FlexibilityIri;
impl PartialEq<&str> for FlexibilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLEXIBILITY_IRI_HTTP || *other == FLEXIBILITY_IRI_HTTPS
	}
}
impl PartialEq<FlexibilityIri> for &str {
	fn eq(&self, other: &FlexibilityIri) -> bool {
		*self == FLEXIBILITY_IRI_HTTP || *self == FLEXIBILITY_IRI_HTTPS
	}
}
pub struct FlexibilityIriOrLabel;
impl PartialEq<&str> for FlexibilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FlexibilityIri || *other == FLEXIBILITY_LABEL
	}
}
impl PartialEq<FlexibilityIriOrLabel> for &str {
	fn eq(&self, other: &FlexibilityIriOrLabel) -> bool {
		*self == FlexibilityIri || *self == FLEXIBILITY_LABEL
	}
}
