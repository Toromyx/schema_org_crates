/// <https://schema.org/MedicalCondition>
pub const MEDICAL_CONDITION_IRI_HTTP: &str = "http://schema.org/MedicalCondition";
/// <https://schema.org/MedicalCondition>
pub const MEDICAL_CONDITION_IRI_HTTPS: &str = "https://schema.org/MedicalCondition";
/// <https://schema.org/MedicalCondition>
pub const MEDICAL_CONDITION_LABEL: &str = "MedicalCondition";
pub struct MedicalConditionIri;
impl PartialEq<&str> for MedicalConditionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_CONDITION_IRI_HTTP || *other == MEDICAL_CONDITION_IRI_HTTPS
	}
}
impl PartialEq<MedicalConditionIri> for &str {
	fn eq(&self, other: &MedicalConditionIri) -> bool {
		*self == MEDICAL_CONDITION_IRI_HTTP || *self == MEDICAL_CONDITION_IRI_HTTPS
	}
}
pub struct MedicalConditionIriOrLabel;
impl PartialEq<&str> for MedicalConditionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalConditionIri || *other == MEDICAL_CONDITION_LABEL
	}
}
impl PartialEq<MedicalConditionIriOrLabel> for &str {
	fn eq(&self, other: &MedicalConditionIriOrLabel) -> bool {
		*self == MedicalConditionIri || *self == MEDICAL_CONDITION_LABEL
	}
}
