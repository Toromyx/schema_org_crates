/// <https://schema.org/MedicalEntity>
pub const MEDICAL_ENTITY_IRI_HTTP: &str = "http://schema.org/MedicalEntity";
/// <https://schema.org/MedicalEntity>
pub const MEDICAL_ENTITY_IRI_HTTPS: &str = "https://schema.org/MedicalEntity";
/// <https://schema.org/MedicalEntity>
pub const MEDICAL_ENTITY_LABEL: &str = "MedicalEntity";
pub struct MedicalEntityIri;
impl PartialEq<&str> for MedicalEntityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_ENTITY_IRI_HTTP || *other == MEDICAL_ENTITY_IRI_HTTPS
	}
}
impl PartialEq<MedicalEntityIri> for &str {
	fn eq(&self, other: &MedicalEntityIri) -> bool {
		*self == MEDICAL_ENTITY_IRI_HTTP || *self == MEDICAL_ENTITY_IRI_HTTPS
	}
}
pub struct MedicalEntityIriOrLabel;
impl PartialEq<&str> for MedicalEntityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalEntityIri || *other == MEDICAL_ENTITY_LABEL
	}
}
impl PartialEq<MedicalEntityIriOrLabel> for &str {
	fn eq(&self, other: &MedicalEntityIriOrLabel) -> bool {
		*self == MedicalEntityIri || *self == MEDICAL_ENTITY_LABEL
	}
}
