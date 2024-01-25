/// <https://schema.org/SideEffectsHealthAspect>
pub const SIDE_EFFECTS_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/SideEffectsHealthAspect";
/// <https://schema.org/SideEffectsHealthAspect>
pub const SIDE_EFFECTS_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/SideEffectsHealthAspect";
/// <https://schema.org/SideEffectsHealthAspect>
pub const SIDE_EFFECTS_HEALTH_ASPECT_LABEL: &str = "SideEffectsHealthAspect";
pub struct SideEffectsHealthAspectIri;
impl PartialEq<&str> for SideEffectsHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIDE_EFFECTS_HEALTH_ASPECT_IRI_HTTP
			|| *other == SIDE_EFFECTS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<SideEffectsHealthAspectIri> for &str {
	fn eq(&self, other: &SideEffectsHealthAspectIri) -> bool {
		*self == SIDE_EFFECTS_HEALTH_ASPECT_IRI_HTTP
			|| *self == SIDE_EFFECTS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct SideEffectsHealthAspectIriOrLabel;
impl PartialEq<&str> for SideEffectsHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SideEffectsHealthAspectIri || *other == SIDE_EFFECTS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<SideEffectsHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &SideEffectsHealthAspectIriOrLabel) -> bool {
		*self == SideEffectsHealthAspectIri || *self == SIDE_EFFECTS_HEALTH_ASPECT_LABEL
	}
}
