/// <https://schema.org/SymptomsHealthAspect>
pub const SYMPTOMS_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/SymptomsHealthAspect";
/// <https://schema.org/SymptomsHealthAspect>
pub const SYMPTOMS_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/SymptomsHealthAspect";
/// <https://schema.org/SymptomsHealthAspect>
pub const SYMPTOMS_HEALTH_ASPECT_LABEL: &str = "SymptomsHealthAspect";
pub struct SymptomsHealthAspectIri;
impl PartialEq<&str> for SymptomsHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SYMPTOMS_HEALTH_ASPECT_IRI_HTTP || *other == SYMPTOMS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<SymptomsHealthAspectIri> for &str {
	fn eq(&self, other: &SymptomsHealthAspectIri) -> bool {
		*self == SYMPTOMS_HEALTH_ASPECT_IRI_HTTP || *self == SYMPTOMS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct SymptomsHealthAspectIriOrLabel;
impl PartialEq<&str> for SymptomsHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SymptomsHealthAspectIri || *other == SYMPTOMS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<SymptomsHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &SymptomsHealthAspectIriOrLabel) -> bool {
		*self == SymptomsHealthAspectIri || *self == SYMPTOMS_HEALTH_ASPECT_LABEL
	}
}
