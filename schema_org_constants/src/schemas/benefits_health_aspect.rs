/// <https://schema.org/BenefitsHealthAspect>
pub const BENEFITS_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/BenefitsHealthAspect";
/// <https://schema.org/BenefitsHealthAspect>
pub const BENEFITS_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/BenefitsHealthAspect";
/// <https://schema.org/BenefitsHealthAspect>
pub const BENEFITS_HEALTH_ASPECT_LABEL: &str = "BenefitsHealthAspect";
pub struct BenefitsHealthAspectIri;
impl PartialEq<&str> for BenefitsHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BENEFITS_HEALTH_ASPECT_IRI_HTTP || *other == BENEFITS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<BenefitsHealthAspectIri> for &str {
	fn eq(&self, other: &BenefitsHealthAspectIri) -> bool {
		*self == BENEFITS_HEALTH_ASPECT_IRI_HTTP || *self == BENEFITS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct BenefitsHealthAspectIriOrLabel;
impl PartialEq<&str> for BenefitsHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BenefitsHealthAspectIri || *other == BENEFITS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<BenefitsHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &BenefitsHealthAspectIriOrLabel) -> bool {
		*self == BenefitsHealthAspectIri || *self == BENEFITS_HEALTH_ASPECT_LABEL
	}
}
