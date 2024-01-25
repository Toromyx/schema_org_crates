/// <https://schema.org/Specialty>
pub const SPECIALTY_IRI_HTTP: &str = "http://schema.org/Specialty";
/// <https://schema.org/Specialty>
pub const SPECIALTY_IRI_HTTPS: &str = "https://schema.org/Specialty";
/// <https://schema.org/Specialty>
pub const SPECIALTY_LABEL: &str = "Specialty";
pub struct SpecialtyIri;
impl PartialEq<&str> for SpecialtyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPECIALTY_IRI_HTTP || *other == SPECIALTY_IRI_HTTPS
	}
}
impl PartialEq<SpecialtyIri> for &str {
	fn eq(&self, other: &SpecialtyIri) -> bool {
		*self == SPECIALTY_IRI_HTTP || *self == SPECIALTY_IRI_HTTPS
	}
}
pub struct SpecialtyIriOrLabel;
impl PartialEq<&str> for SpecialtyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpecialtyIri || *other == SPECIALTY_LABEL
	}
}
impl PartialEq<SpecialtyIriOrLabel> for &str {
	fn eq(&self, other: &SpecialtyIriOrLabel) -> bool {
		*self == SpecialtyIri || *self == SPECIALTY_LABEL
	}
}
