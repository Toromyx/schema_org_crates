/// <https://schema.org/specialty>
pub const SPECIALTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/specialty";
/// <https://schema.org/specialty>
pub const SPECIALTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/specialty";
/// <https://schema.org/specialty>
pub const SPECIALTY_PROPERTY_LABEL: &str = "specialty";
pub struct SpecialtyPropertyIri;
impl PartialEq<&str> for SpecialtyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPECIALTY_PROPERTY_IRI_HTTP || *other == SPECIALTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpecialtyPropertyIri> for &str {
	fn eq(&self, other: &SpecialtyPropertyIri) -> bool {
		*self == SPECIALTY_PROPERTY_IRI_HTTP || *self == SPECIALTY_PROPERTY_IRI_HTTPS
	}
}
pub struct SpecialtyPropertyIriOrLabel;
impl PartialEq<&str> for SpecialtyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpecialtyPropertyIri || *other == SPECIALTY_PROPERTY_LABEL
	}
}
impl PartialEq<SpecialtyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpecialtyPropertyIriOrLabel) -> bool {
		*self == SpecialtyPropertyIri || *self == SPECIALTY_PROPERTY_LABEL
	}
}
