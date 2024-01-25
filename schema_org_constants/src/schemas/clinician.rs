/// <https://schema.org/Clinician>
pub const CLINICIAN_IRI_HTTP: &str = "http://schema.org/Clinician";
/// <https://schema.org/Clinician>
pub const CLINICIAN_IRI_HTTPS: &str = "https://schema.org/Clinician";
/// <https://schema.org/Clinician>
pub const CLINICIAN_LABEL: &str = "Clinician";
pub struct ClinicianIri;
impl PartialEq<&str> for ClinicianIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLINICIAN_IRI_HTTP || *other == CLINICIAN_IRI_HTTPS
	}
}
impl PartialEq<ClinicianIri> for &str {
	fn eq(&self, other: &ClinicianIri) -> bool {
		*self == CLINICIAN_IRI_HTTP || *self == CLINICIAN_IRI_HTTPS
	}
}
pub struct ClinicianIriOrLabel;
impl PartialEq<&str> for ClinicianIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClinicianIri || *other == CLINICIAN_LABEL
	}
}
impl PartialEq<ClinicianIriOrLabel> for &str {
	fn eq(&self, other: &ClinicianIriOrLabel) -> bool {
		*self == ClinicianIri || *self == CLINICIAN_LABEL
	}
}
