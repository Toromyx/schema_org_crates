/// <https://schema.org/PET>
pub const PET_IRI_HTTP: &str = "http://schema.org/PET";
/// <https://schema.org/PET>
pub const PET_IRI_HTTPS: &str = "https://schema.org/PET";
/// <https://schema.org/PET>
pub const PET_LABEL: &str = "PET";
pub struct PetIri;
impl PartialEq<&str> for PetIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PET_IRI_HTTP || *other == PET_IRI_HTTPS
	}
}
impl PartialEq<PetIri> for &str {
	fn eq(&self, other: &PetIri) -> bool {
		*self == PET_IRI_HTTP || *self == PET_IRI_HTTPS
	}
}
pub struct PetIriOrLabel;
impl PartialEq<&str> for PetIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PetIri || *other == PET_LABEL
	}
}
impl PartialEq<PetIriOrLabel> for &str {
	fn eq(&self, other: &PetIriOrLabel) -> bool {
		*self == PetIri || *self == PET_LABEL
	}
}
