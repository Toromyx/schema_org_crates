/// <https://schema.org/MedicalWebPage>
pub const MEDICAL_WEB_PAGE_IRI_HTTP: &str = "http://schema.org/MedicalWebPage";
/// <https://schema.org/MedicalWebPage>
pub const MEDICAL_WEB_PAGE_IRI_HTTPS: &str = "https://schema.org/MedicalWebPage";
/// <https://schema.org/MedicalWebPage>
pub const MEDICAL_WEB_PAGE_LABEL: &str = "MedicalWebPage";
pub struct MedicalWebPageIri;
impl PartialEq<&str> for MedicalWebPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_WEB_PAGE_IRI_HTTP || *other == MEDICAL_WEB_PAGE_IRI_HTTPS
	}
}
impl PartialEq<MedicalWebPageIri> for &str {
	fn eq(&self, other: &MedicalWebPageIri) -> bool {
		*self == MEDICAL_WEB_PAGE_IRI_HTTP || *self == MEDICAL_WEB_PAGE_IRI_HTTPS
	}
}
pub struct MedicalWebPageIriOrLabel;
impl PartialEq<&str> for MedicalWebPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalWebPageIri || *other == MEDICAL_WEB_PAGE_LABEL
	}
}
impl PartialEq<MedicalWebPageIriOrLabel> for &str {
	fn eq(&self, other: &MedicalWebPageIriOrLabel) -> bool {
		*self == MedicalWebPageIri || *self == MEDICAL_WEB_PAGE_LABEL
	}
}
