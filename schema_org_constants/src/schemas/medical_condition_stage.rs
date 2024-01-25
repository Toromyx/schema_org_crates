/// <https://schema.org/MedicalConditionStage>
pub const MEDICAL_CONDITION_STAGE_IRI_HTTP: &str = "http://schema.org/MedicalConditionStage";
/// <https://schema.org/MedicalConditionStage>
pub const MEDICAL_CONDITION_STAGE_IRI_HTTPS: &str = "https://schema.org/MedicalConditionStage";
/// <https://schema.org/MedicalConditionStage>
pub const MEDICAL_CONDITION_STAGE_LABEL: &str = "MedicalConditionStage";
pub struct MedicalConditionStageIri;
impl PartialEq<&str> for MedicalConditionStageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_CONDITION_STAGE_IRI_HTTP || *other == MEDICAL_CONDITION_STAGE_IRI_HTTPS
	}
}
impl PartialEq<MedicalConditionStageIri> for &str {
	fn eq(&self, other: &MedicalConditionStageIri) -> bool {
		*self == MEDICAL_CONDITION_STAGE_IRI_HTTP || *self == MEDICAL_CONDITION_STAGE_IRI_HTTPS
	}
}
pub struct MedicalConditionStageIriOrLabel;
impl PartialEq<&str> for MedicalConditionStageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalConditionStageIri || *other == MEDICAL_CONDITION_STAGE_LABEL
	}
}
impl PartialEq<MedicalConditionStageIriOrLabel> for &str {
	fn eq(&self, other: &MedicalConditionStageIriOrLabel) -> bool {
		*self == MedicalConditionStageIri || *self == MEDICAL_CONDITION_STAGE_LABEL
	}
}
