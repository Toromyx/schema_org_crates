/// <https://schema.org/diagnosis>
pub const DIAGNOSIS_PROPERTY_IRI_HTTP: &str = "http://schema.org/diagnosis";
/// <https://schema.org/diagnosis>
pub const DIAGNOSIS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/diagnosis";
/// <https://schema.org/diagnosis>
pub const DIAGNOSIS_PROPERTY_LABEL: &str = "diagnosis";
pub struct DiagnosisPropertyIri;
impl PartialEq<&str> for DiagnosisPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIAGNOSIS_PROPERTY_IRI_HTTP || *other == DIAGNOSIS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiagnosisPropertyIri> for &str {
	fn eq(&self, other: &DiagnosisPropertyIri) -> bool {
		*self == DIAGNOSIS_PROPERTY_IRI_HTTP || *self == DIAGNOSIS_PROPERTY_IRI_HTTPS
	}
}
pub struct DiagnosisPropertyIriOrLabel;
impl PartialEq<&str> for DiagnosisPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiagnosisPropertyIri || *other == DIAGNOSIS_PROPERTY_LABEL
	}
}
impl PartialEq<DiagnosisPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiagnosisPropertyIriOrLabel) -> bool {
		*self == DiagnosisPropertyIri || *self == DIAGNOSIS_PROPERTY_LABEL
	}
}
