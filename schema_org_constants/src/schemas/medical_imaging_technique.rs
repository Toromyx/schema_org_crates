/// <https://schema.org/MedicalImagingTechnique>
pub const MEDICAL_IMAGING_TECHNIQUE_IRI_HTTP: &str = "http://schema.org/MedicalImagingTechnique";
/// <https://schema.org/MedicalImagingTechnique>
pub const MEDICAL_IMAGING_TECHNIQUE_IRI_HTTPS: &str = "https://schema.org/MedicalImagingTechnique";
/// <https://schema.org/MedicalImagingTechnique>
pub const MEDICAL_IMAGING_TECHNIQUE_LABEL: &str = "MedicalImagingTechnique";
pub struct MedicalImagingTechniqueIri;
impl PartialEq<&str> for MedicalImagingTechniqueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_IMAGING_TECHNIQUE_IRI_HTTP
			|| *other == MEDICAL_IMAGING_TECHNIQUE_IRI_HTTPS
	}
}
impl PartialEq<MedicalImagingTechniqueIri> for &str {
	fn eq(&self, other: &MedicalImagingTechniqueIri) -> bool {
		*self == MEDICAL_IMAGING_TECHNIQUE_IRI_HTTP || *self == MEDICAL_IMAGING_TECHNIQUE_IRI_HTTPS
	}
}
pub struct MedicalImagingTechniqueIriOrLabel;
impl PartialEq<&str> for MedicalImagingTechniqueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalImagingTechniqueIri || *other == MEDICAL_IMAGING_TECHNIQUE_LABEL
	}
}
impl PartialEq<MedicalImagingTechniqueIriOrLabel> for &str {
	fn eq(&self, other: &MedicalImagingTechniqueIriOrLabel) -> bool {
		*self == MedicalImagingTechniqueIri || *self == MEDICAL_IMAGING_TECHNIQUE_LABEL
	}
}
