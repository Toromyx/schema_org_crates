/// <https://schema.org/ScreeningHealthAspect>
pub const SCREENING_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/ScreeningHealthAspect";
/// <https://schema.org/ScreeningHealthAspect>
pub const SCREENING_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/ScreeningHealthAspect";
/// <https://schema.org/ScreeningHealthAspect>
pub const SCREENING_HEALTH_ASPECT_LABEL: &str = "ScreeningHealthAspect";
pub struct ScreeningHealthAspectIri;
impl PartialEq<&str> for ScreeningHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCREENING_HEALTH_ASPECT_IRI_HTTP || *other == SCREENING_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<ScreeningHealthAspectIri> for &str {
	fn eq(&self, other: &ScreeningHealthAspectIri) -> bool {
		*self == SCREENING_HEALTH_ASPECT_IRI_HTTP || *self == SCREENING_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct ScreeningHealthAspectIriOrLabel;
impl PartialEq<&str> for ScreeningHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScreeningHealthAspectIri || *other == SCREENING_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<ScreeningHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &ScreeningHealthAspectIriOrLabel) -> bool {
		*self == ScreeningHealthAspectIri || *self == SCREENING_HEALTH_ASPECT_LABEL
	}
}
