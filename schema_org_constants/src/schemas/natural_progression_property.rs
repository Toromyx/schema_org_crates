/// <https://schema.org/naturalProgression>
pub const NATURAL_PROGRESSION_PROPERTY_IRI_HTTP: &str = "http://schema.org/naturalProgression";
/// <https://schema.org/naturalProgression>
pub const NATURAL_PROGRESSION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/naturalProgression";
/// <https://schema.org/naturalProgression>
pub const NATURAL_PROGRESSION_PROPERTY_LABEL: &str = "naturalProgression";
pub struct NaturalProgressionPropertyIri;
impl PartialEq<&str> for NaturalProgressionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NATURAL_PROGRESSION_PROPERTY_IRI_HTTP
			|| *other == NATURAL_PROGRESSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NaturalProgressionPropertyIri> for &str {
	fn eq(&self, other: &NaturalProgressionPropertyIri) -> bool {
		*self == NATURAL_PROGRESSION_PROPERTY_IRI_HTTP
			|| *self == NATURAL_PROGRESSION_PROPERTY_IRI_HTTPS
	}
}
pub struct NaturalProgressionPropertyIriOrLabel;
impl PartialEq<&str> for NaturalProgressionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NaturalProgressionPropertyIri || *other == NATURAL_PROGRESSION_PROPERTY_LABEL
	}
}
impl PartialEq<NaturalProgressionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NaturalProgressionPropertyIriOrLabel) -> bool {
		*self == NaturalProgressionPropertyIri || *self == NATURAL_PROGRESSION_PROPERTY_LABEL
	}
}
