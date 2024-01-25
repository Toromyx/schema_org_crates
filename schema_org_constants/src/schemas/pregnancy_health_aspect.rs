/// <https://schema.org/PregnancyHealthAspect>
pub const PREGNANCY_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/PregnancyHealthAspect";
/// <https://schema.org/PregnancyHealthAspect>
pub const PREGNANCY_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/PregnancyHealthAspect";
/// <https://schema.org/PregnancyHealthAspect>
pub const PREGNANCY_HEALTH_ASPECT_LABEL: &str = "PregnancyHealthAspect";
pub struct PregnancyHealthAspectIri;
impl PartialEq<&str> for PregnancyHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREGNANCY_HEALTH_ASPECT_IRI_HTTP || *other == PREGNANCY_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<PregnancyHealthAspectIri> for &str {
	fn eq(&self, other: &PregnancyHealthAspectIri) -> bool {
		*self == PREGNANCY_HEALTH_ASPECT_IRI_HTTP || *self == PREGNANCY_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct PregnancyHealthAspectIriOrLabel;
impl PartialEq<&str> for PregnancyHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PregnancyHealthAspectIri || *other == PREGNANCY_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<PregnancyHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &PregnancyHealthAspectIriOrLabel) -> bool {
		*self == PregnancyHealthAspectIri || *self == PREGNANCY_HEALTH_ASPECT_LABEL
	}
}
