/// <https://schema.org/SafetyHealthAspect>
pub const SAFETY_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/SafetyHealthAspect";
/// <https://schema.org/SafetyHealthAspect>
pub const SAFETY_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/SafetyHealthAspect";
/// <https://schema.org/SafetyHealthAspect>
pub const SAFETY_HEALTH_ASPECT_LABEL: &str = "SafetyHealthAspect";
pub struct SafetyHealthAspectIri;
impl PartialEq<&str> for SafetyHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SAFETY_HEALTH_ASPECT_IRI_HTTP || *other == SAFETY_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<SafetyHealthAspectIri> for &str {
	fn eq(&self, other: &SafetyHealthAspectIri) -> bool {
		*self == SAFETY_HEALTH_ASPECT_IRI_HTTP || *self == SAFETY_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct SafetyHealthAspectIriOrLabel;
impl PartialEq<&str> for SafetyHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SafetyHealthAspectIri || *other == SAFETY_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<SafetyHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &SafetyHealthAspectIriOrLabel) -> bool {
		*self == SafetyHealthAspectIri || *self == SAFETY_HEALTH_ASPECT_LABEL
	}
}
