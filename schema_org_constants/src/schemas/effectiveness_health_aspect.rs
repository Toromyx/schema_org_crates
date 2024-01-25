/// <https://schema.org/EffectivenessHealthAspect>
pub const EFFECTIVENESS_HEALTH_ASPECT_IRI_HTTP: &str =
	"http://schema.org/EffectivenessHealthAspect";
/// <https://schema.org/EffectivenessHealthAspect>
pub const EFFECTIVENESS_HEALTH_ASPECT_IRI_HTTPS: &str =
	"https://schema.org/EffectivenessHealthAspect";
/// <https://schema.org/EffectivenessHealthAspect>
pub const EFFECTIVENESS_HEALTH_ASPECT_LABEL: &str = "EffectivenessHealthAspect";
pub struct EffectivenessHealthAspectIri;
impl PartialEq<&str> for EffectivenessHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EFFECTIVENESS_HEALTH_ASPECT_IRI_HTTP
			|| *other == EFFECTIVENESS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<EffectivenessHealthAspectIri> for &str {
	fn eq(&self, other: &EffectivenessHealthAspectIri) -> bool {
		*self == EFFECTIVENESS_HEALTH_ASPECT_IRI_HTTP
			|| *self == EFFECTIVENESS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct EffectivenessHealthAspectIriOrLabel;
impl PartialEq<&str> for EffectivenessHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EffectivenessHealthAspectIri || *other == EFFECTIVENESS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<EffectivenessHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &EffectivenessHealthAspectIriOrLabel) -> bool {
		*self == EffectivenessHealthAspectIri || *self == EFFECTIVENESS_HEALTH_ASPECT_LABEL
	}
}
