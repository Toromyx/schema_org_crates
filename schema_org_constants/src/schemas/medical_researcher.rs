/// <https://schema.org/MedicalResearcher>
pub const MEDICAL_RESEARCHER_IRI_HTTP: &str = "http://schema.org/MedicalResearcher";
/// <https://schema.org/MedicalResearcher>
pub const MEDICAL_RESEARCHER_IRI_HTTPS: &str = "https://schema.org/MedicalResearcher";
/// <https://schema.org/MedicalResearcher>
pub const MEDICAL_RESEARCHER_LABEL: &str = "MedicalResearcher";
pub struct MedicalResearcherIri;
impl PartialEq<&str> for MedicalResearcherIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_RESEARCHER_IRI_HTTP || *other == MEDICAL_RESEARCHER_IRI_HTTPS
	}
}
impl PartialEq<MedicalResearcherIri> for &str {
	fn eq(&self, other: &MedicalResearcherIri) -> bool {
		*self == MEDICAL_RESEARCHER_IRI_HTTP || *self == MEDICAL_RESEARCHER_IRI_HTTPS
	}
}
pub struct MedicalResearcherIriOrLabel;
impl PartialEq<&str> for MedicalResearcherIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalResearcherIri || *other == MEDICAL_RESEARCHER_LABEL
	}
}
impl PartialEq<MedicalResearcherIriOrLabel> for &str {
	fn eq(&self, other: &MedicalResearcherIriOrLabel) -> bool {
		*self == MedicalResearcherIri || *self == MEDICAL_RESEARCHER_LABEL
	}
}
