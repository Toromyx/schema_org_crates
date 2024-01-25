/// <https://schema.org/MedicalTestPanel>
pub const MEDICAL_TEST_PANEL_IRI_HTTP: &str = "http://schema.org/MedicalTestPanel";
/// <https://schema.org/MedicalTestPanel>
pub const MEDICAL_TEST_PANEL_IRI_HTTPS: &str = "https://schema.org/MedicalTestPanel";
/// <https://schema.org/MedicalTestPanel>
pub const MEDICAL_TEST_PANEL_LABEL: &str = "MedicalTestPanel";
pub struct MedicalTestPanelIri;
impl PartialEq<&str> for MedicalTestPanelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_TEST_PANEL_IRI_HTTP || *other == MEDICAL_TEST_PANEL_IRI_HTTPS
	}
}
impl PartialEq<MedicalTestPanelIri> for &str {
	fn eq(&self, other: &MedicalTestPanelIri) -> bool {
		*self == MEDICAL_TEST_PANEL_IRI_HTTP || *self == MEDICAL_TEST_PANEL_IRI_HTTPS
	}
}
pub struct MedicalTestPanelIriOrLabel;
impl PartialEq<&str> for MedicalTestPanelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalTestPanelIri || *other == MEDICAL_TEST_PANEL_LABEL
	}
}
impl PartialEq<MedicalTestPanelIriOrLabel> for &str {
	fn eq(&self, other: &MedicalTestPanelIriOrLabel) -> bool {
		*self == MedicalTestPanelIri || *self == MEDICAL_TEST_PANEL_LABEL
	}
}
