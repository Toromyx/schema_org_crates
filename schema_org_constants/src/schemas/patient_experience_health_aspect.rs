/// <https://schema.org/PatientExperienceHealthAspect>
pub const PATIENT_EXPERIENCE_HEALTH_ASPECT_IRI_HTTP: &str =
	"http://schema.org/PatientExperienceHealthAspect";
/// <https://schema.org/PatientExperienceHealthAspect>
pub const PATIENT_EXPERIENCE_HEALTH_ASPECT_IRI_HTTPS: &str =
	"https://schema.org/PatientExperienceHealthAspect";
/// <https://schema.org/PatientExperienceHealthAspect>
pub const PATIENT_EXPERIENCE_HEALTH_ASPECT_LABEL: &str = "PatientExperienceHealthAspect";
pub struct PatientExperienceHealthAspectIri;
impl PartialEq<&str> for PatientExperienceHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PATIENT_EXPERIENCE_HEALTH_ASPECT_IRI_HTTP
			|| *other == PATIENT_EXPERIENCE_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<PatientExperienceHealthAspectIri> for &str {
	fn eq(&self, other: &PatientExperienceHealthAspectIri) -> bool {
		*self == PATIENT_EXPERIENCE_HEALTH_ASPECT_IRI_HTTP
			|| *self == PATIENT_EXPERIENCE_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct PatientExperienceHealthAspectIriOrLabel;
impl PartialEq<&str> for PatientExperienceHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PatientExperienceHealthAspectIri
			|| *other == PATIENT_EXPERIENCE_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<PatientExperienceHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &PatientExperienceHealthAspectIriOrLabel) -> bool {
		*self == PatientExperienceHealthAspectIri || *self == PATIENT_EXPERIENCE_HEALTH_ASPECT_LABEL
	}
}
