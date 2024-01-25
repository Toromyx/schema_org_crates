/// <https://schema.org/PreventionHealthAspect>
pub const PREVENTION_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/PreventionHealthAspect";
/// <https://schema.org/PreventionHealthAspect>
pub const PREVENTION_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/PreventionHealthAspect";
/// <https://schema.org/PreventionHealthAspect>
pub const PREVENTION_HEALTH_ASPECT_LABEL: &str = "PreventionHealthAspect";
pub struct PreventionHealthAspectIri;
impl PartialEq<&str> for PreventionHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREVENTION_HEALTH_ASPECT_IRI_HTTP || *other == PREVENTION_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<PreventionHealthAspectIri> for &str {
	fn eq(&self, other: &PreventionHealthAspectIri) -> bool {
		*self == PREVENTION_HEALTH_ASPECT_IRI_HTTP || *self == PREVENTION_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct PreventionHealthAspectIriOrLabel;
impl PartialEq<&str> for PreventionHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreventionHealthAspectIri || *other == PREVENTION_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<PreventionHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &PreventionHealthAspectIriOrLabel) -> bool {
		*self == PreventionHealthAspectIri || *self == PREVENTION_HEALTH_ASPECT_LABEL
	}
}
