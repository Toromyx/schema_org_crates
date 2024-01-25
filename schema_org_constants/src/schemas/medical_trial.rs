/// <https://schema.org/MedicalTrial>
pub const MEDICAL_TRIAL_IRI_HTTP: &str = "http://schema.org/MedicalTrial";
/// <https://schema.org/MedicalTrial>
pub const MEDICAL_TRIAL_IRI_HTTPS: &str = "https://schema.org/MedicalTrial";
/// <https://schema.org/MedicalTrial>
pub const MEDICAL_TRIAL_LABEL: &str = "MedicalTrial";
pub struct MedicalTrialIri;
impl PartialEq<&str> for MedicalTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_TRIAL_IRI_HTTP || *other == MEDICAL_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<MedicalTrialIri> for &str {
	fn eq(&self, other: &MedicalTrialIri) -> bool {
		*self == MEDICAL_TRIAL_IRI_HTTP || *self == MEDICAL_TRIAL_IRI_HTTPS
	}
}
pub struct MedicalTrialIriOrLabel;
impl PartialEq<&str> for MedicalTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalTrialIri || *other == MEDICAL_TRIAL_LABEL
	}
}
impl PartialEq<MedicalTrialIriOrLabel> for &str {
	fn eq(&self, other: &MedicalTrialIriOrLabel) -> bool {
		*self == MedicalTrialIri || *self == MEDICAL_TRIAL_LABEL
	}
}
