/// <https://schema.org/LivingWithHealthAspect>
pub const LIVING_WITH_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/LivingWithHealthAspect";
/// <https://schema.org/LivingWithHealthAspect>
pub const LIVING_WITH_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/LivingWithHealthAspect";
/// <https://schema.org/LivingWithHealthAspect>
pub const LIVING_WITH_HEALTH_ASPECT_LABEL: &str = "LivingWithHealthAspect";
pub struct LivingWithHealthAspectIri;
impl PartialEq<&str> for LivingWithHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIVING_WITH_HEALTH_ASPECT_IRI_HTTP
			|| *other == LIVING_WITH_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<LivingWithHealthAspectIri> for &str {
	fn eq(&self, other: &LivingWithHealthAspectIri) -> bool {
		*self == LIVING_WITH_HEALTH_ASPECT_IRI_HTTP || *self == LIVING_WITH_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct LivingWithHealthAspectIriOrLabel;
impl PartialEq<&str> for LivingWithHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LivingWithHealthAspectIri || *other == LIVING_WITH_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<LivingWithHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &LivingWithHealthAspectIriOrLabel) -> bool {
		*self == LivingWithHealthAspectIri || *self == LIVING_WITH_HEALTH_ASPECT_LABEL
	}
}
