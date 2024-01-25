/// <https://schema.org/SelfCareHealthAspect>
pub const SELF_CARE_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/SelfCareHealthAspect";
/// <https://schema.org/SelfCareHealthAspect>
pub const SELF_CARE_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/SelfCareHealthAspect";
/// <https://schema.org/SelfCareHealthAspect>
pub const SELF_CARE_HEALTH_ASPECT_LABEL: &str = "SelfCareHealthAspect";
pub struct SelfCareHealthAspectIri;
impl PartialEq<&str> for SelfCareHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SELF_CARE_HEALTH_ASPECT_IRI_HTTP || *other == SELF_CARE_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<SelfCareHealthAspectIri> for &str {
	fn eq(&self, other: &SelfCareHealthAspectIri) -> bool {
		*self == SELF_CARE_HEALTH_ASPECT_IRI_HTTP || *self == SELF_CARE_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct SelfCareHealthAspectIriOrLabel;
impl PartialEq<&str> for SelfCareHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SelfCareHealthAspectIri || *other == SELF_CARE_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<SelfCareHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &SelfCareHealthAspectIriOrLabel) -> bool {
		*self == SelfCareHealthAspectIri || *self == SELF_CARE_HEALTH_ASPECT_LABEL
	}
}
