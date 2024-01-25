/// <https://schema.org/StagesHealthAspect>
pub const STAGES_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/StagesHealthAspect";
/// <https://schema.org/StagesHealthAspect>
pub const STAGES_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/StagesHealthAspect";
/// <https://schema.org/StagesHealthAspect>
pub const STAGES_HEALTH_ASPECT_LABEL: &str = "StagesHealthAspect";
pub struct StagesHealthAspectIri;
impl PartialEq<&str> for StagesHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STAGES_HEALTH_ASPECT_IRI_HTTP || *other == STAGES_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<StagesHealthAspectIri> for &str {
	fn eq(&self, other: &StagesHealthAspectIri) -> bool {
		*self == STAGES_HEALTH_ASPECT_IRI_HTTP || *self == STAGES_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct StagesHealthAspectIriOrLabel;
impl PartialEq<&str> for StagesHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StagesHealthAspectIri || *other == STAGES_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<StagesHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &StagesHealthAspectIriOrLabel) -> bool {
		*self == StagesHealthAspectIri || *self == STAGES_HEALTH_ASPECT_LABEL
	}
}
