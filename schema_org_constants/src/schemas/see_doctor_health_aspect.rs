/// <https://schema.org/SeeDoctorHealthAspect>
pub const SEE_DOCTOR_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/SeeDoctorHealthAspect";
/// <https://schema.org/SeeDoctorHealthAspect>
pub const SEE_DOCTOR_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/SeeDoctorHealthAspect";
/// <https://schema.org/SeeDoctorHealthAspect>
pub const SEE_DOCTOR_HEALTH_ASPECT_LABEL: &str = "SeeDoctorHealthAspect";
pub struct SeeDoctorHealthAspectIri;
impl PartialEq<&str> for SeeDoctorHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEE_DOCTOR_HEALTH_ASPECT_IRI_HTTP || *other == SEE_DOCTOR_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<SeeDoctorHealthAspectIri> for &str {
	fn eq(&self, other: &SeeDoctorHealthAspectIri) -> bool {
		*self == SEE_DOCTOR_HEALTH_ASPECT_IRI_HTTP || *self == SEE_DOCTOR_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct SeeDoctorHealthAspectIriOrLabel;
impl PartialEq<&str> for SeeDoctorHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeeDoctorHealthAspectIri || *other == SEE_DOCTOR_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<SeeDoctorHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &SeeDoctorHealthAspectIriOrLabel) -> bool {
		*self == SeeDoctorHealthAspectIri || *self == SEE_DOCTOR_HEALTH_ASPECT_LABEL
	}
}
