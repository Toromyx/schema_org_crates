/// <https://schema.org/Patient>
pub const PATIENT_IRI_HTTP: &str = "http://schema.org/Patient";
/// <https://schema.org/Patient>
pub const PATIENT_IRI_HTTPS: &str = "https://schema.org/Patient";
/// <https://schema.org/Patient>
pub const PATIENT_LABEL: &str = "Patient";
pub struct PatientIri;
impl PartialEq<&str> for PatientIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PATIENT_IRI_HTTP || *other == PATIENT_IRI_HTTPS
	}
}
impl PartialEq<PatientIri> for &str {
	fn eq(&self, other: &PatientIri) -> bool {
		*self == PATIENT_IRI_HTTP || *self == PATIENT_IRI_HTTPS
	}
}
pub struct PatientIriOrLabel;
impl PartialEq<&str> for PatientIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PatientIri || *other == PATIENT_LABEL
	}
}
impl PartialEq<PatientIriOrLabel> for &str {
	fn eq(&self, other: &PatientIriOrLabel) -> bool {
		*self == PatientIri || *self == PATIENT_LABEL
	}
}
