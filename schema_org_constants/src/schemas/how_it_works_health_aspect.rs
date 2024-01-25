/// <https://schema.org/HowItWorksHealthAspect>
pub const HOW_IT_WORKS_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/HowItWorksHealthAspect";
/// <https://schema.org/HowItWorksHealthAspect>
pub const HOW_IT_WORKS_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/HowItWorksHealthAspect";
/// <https://schema.org/HowItWorksHealthAspect>
pub const HOW_IT_WORKS_HEALTH_ASPECT_LABEL: &str = "HowItWorksHealthAspect";
pub struct HowItWorksHealthAspectIri;
impl PartialEq<&str> for HowItWorksHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_IT_WORKS_HEALTH_ASPECT_IRI_HTTP
			|| *other == HOW_IT_WORKS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<HowItWorksHealthAspectIri> for &str {
	fn eq(&self, other: &HowItWorksHealthAspectIri) -> bool {
		*self == HOW_IT_WORKS_HEALTH_ASPECT_IRI_HTTP
			|| *self == HOW_IT_WORKS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct HowItWorksHealthAspectIriOrLabel;
impl PartialEq<&str> for HowItWorksHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowItWorksHealthAspectIri || *other == HOW_IT_WORKS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<HowItWorksHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &HowItWorksHealthAspectIriOrLabel) -> bool {
		*self == HowItWorksHealthAspectIri || *self == HOW_IT_WORKS_HEALTH_ASPECT_LABEL
	}
}
