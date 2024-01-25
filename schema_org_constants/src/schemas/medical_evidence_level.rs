/// <https://schema.org/MedicalEvidenceLevel>
pub const MEDICAL_EVIDENCE_LEVEL_IRI_HTTP: &str = "http://schema.org/MedicalEvidenceLevel";
/// <https://schema.org/MedicalEvidenceLevel>
pub const MEDICAL_EVIDENCE_LEVEL_IRI_HTTPS: &str = "https://schema.org/MedicalEvidenceLevel";
/// <https://schema.org/MedicalEvidenceLevel>
pub const MEDICAL_EVIDENCE_LEVEL_LABEL: &str = "MedicalEvidenceLevel";
pub struct MedicalEvidenceLevelIri;
impl PartialEq<&str> for MedicalEvidenceLevelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_EVIDENCE_LEVEL_IRI_HTTP || *other == MEDICAL_EVIDENCE_LEVEL_IRI_HTTPS
	}
}
impl PartialEq<MedicalEvidenceLevelIri> for &str {
	fn eq(&self, other: &MedicalEvidenceLevelIri) -> bool {
		*self == MEDICAL_EVIDENCE_LEVEL_IRI_HTTP || *self == MEDICAL_EVIDENCE_LEVEL_IRI_HTTPS
	}
}
pub struct MedicalEvidenceLevelIriOrLabel;
impl PartialEq<&str> for MedicalEvidenceLevelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalEvidenceLevelIri || *other == MEDICAL_EVIDENCE_LEVEL_LABEL
	}
}
impl PartialEq<MedicalEvidenceLevelIriOrLabel> for &str {
	fn eq(&self, other: &MedicalEvidenceLevelIriOrLabel) -> bool {
		*self == MedicalEvidenceLevelIri || *self == MEDICAL_EVIDENCE_LEVEL_LABEL
	}
}
