/// <https://schema.org/MedicalTrialDesign>
pub const MEDICAL_TRIAL_DESIGN_IRI_HTTP: &str = "http://schema.org/MedicalTrialDesign";
/// <https://schema.org/MedicalTrialDesign>
pub const MEDICAL_TRIAL_DESIGN_IRI_HTTPS: &str = "https://schema.org/MedicalTrialDesign";
/// <https://schema.org/MedicalTrialDesign>
pub const MEDICAL_TRIAL_DESIGN_LABEL: &str = "MedicalTrialDesign";
pub struct MedicalTrialDesignIri;
impl PartialEq<&str> for MedicalTrialDesignIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_TRIAL_DESIGN_IRI_HTTP || *other == MEDICAL_TRIAL_DESIGN_IRI_HTTPS
	}
}
impl PartialEq<MedicalTrialDesignIri> for &str {
	fn eq(&self, other: &MedicalTrialDesignIri) -> bool {
		*self == MEDICAL_TRIAL_DESIGN_IRI_HTTP || *self == MEDICAL_TRIAL_DESIGN_IRI_HTTPS
	}
}
pub struct MedicalTrialDesignIriOrLabel;
impl PartialEq<&str> for MedicalTrialDesignIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalTrialDesignIri || *other == MEDICAL_TRIAL_DESIGN_LABEL
	}
}
impl PartialEq<MedicalTrialDesignIriOrLabel> for &str {
	fn eq(&self, other: &MedicalTrialDesignIriOrLabel) -> bool {
		*self == MedicalTrialDesignIri || *self == MEDICAL_TRIAL_DESIGN_LABEL
	}
}
