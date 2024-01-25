/// <https://schema.org/MedicalObservationalStudyDesign>
pub const MEDICAL_OBSERVATIONAL_STUDY_DESIGN_IRI_HTTP: &str =
	"http://schema.org/MedicalObservationalStudyDesign";
/// <https://schema.org/MedicalObservationalStudyDesign>
pub const MEDICAL_OBSERVATIONAL_STUDY_DESIGN_IRI_HTTPS: &str =
	"https://schema.org/MedicalObservationalStudyDesign";
/// <https://schema.org/MedicalObservationalStudyDesign>
pub const MEDICAL_OBSERVATIONAL_STUDY_DESIGN_LABEL: &str = "MedicalObservationalStudyDesign";
pub struct MedicalObservationalStudyDesignIri;
impl PartialEq<&str> for MedicalObservationalStudyDesignIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_OBSERVATIONAL_STUDY_DESIGN_IRI_HTTP
			|| *other == MEDICAL_OBSERVATIONAL_STUDY_DESIGN_IRI_HTTPS
	}
}
impl PartialEq<MedicalObservationalStudyDesignIri> for &str {
	fn eq(&self, other: &MedicalObservationalStudyDesignIri) -> bool {
		*self == MEDICAL_OBSERVATIONAL_STUDY_DESIGN_IRI_HTTP
			|| *self == MEDICAL_OBSERVATIONAL_STUDY_DESIGN_IRI_HTTPS
	}
}
pub struct MedicalObservationalStudyDesignIriOrLabel;
impl PartialEq<&str> for MedicalObservationalStudyDesignIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalObservationalStudyDesignIri
			|| *other == MEDICAL_OBSERVATIONAL_STUDY_DESIGN_LABEL
	}
}
impl PartialEq<MedicalObservationalStudyDesignIriOrLabel> for &str {
	fn eq(&self, other: &MedicalObservationalStudyDesignIriOrLabel) -> bool {
		*self == MedicalObservationalStudyDesignIri
			|| *self == MEDICAL_OBSERVATIONAL_STUDY_DESIGN_LABEL
	}
}
