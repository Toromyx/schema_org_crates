/// <https://schema.org/CausesHealthAspect>
pub const CAUSES_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/CausesHealthAspect";
/// <https://schema.org/CausesHealthAspect>
pub const CAUSES_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/CausesHealthAspect";
/// <https://schema.org/CausesHealthAspect>
pub const CAUSES_HEALTH_ASPECT_LABEL: &str = "CausesHealthAspect";
pub struct CausesHealthAspectIri;
impl PartialEq<&str> for CausesHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CAUSES_HEALTH_ASPECT_IRI_HTTP || *other == CAUSES_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<CausesHealthAspectIri> for &str {
	fn eq(&self, other: &CausesHealthAspectIri) -> bool {
		*self == CAUSES_HEALTH_ASPECT_IRI_HTTP || *self == CAUSES_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct CausesHealthAspectIriOrLabel;
impl PartialEq<&str> for CausesHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CausesHealthAspectIri || *other == CAUSES_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<CausesHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &CausesHealthAspectIriOrLabel) -> bool {
		*self == CausesHealthAspectIri || *self == CAUSES_HEALTH_ASPECT_LABEL
	}
}
