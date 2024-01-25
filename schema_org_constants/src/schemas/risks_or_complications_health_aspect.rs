/// <https://schema.org/RisksOrComplicationsHealthAspect>
pub const RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_IRI_HTTP: &str =
	"http://schema.org/RisksOrComplicationsHealthAspect";
/// <https://schema.org/RisksOrComplicationsHealthAspect>
pub const RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_IRI_HTTPS: &str =
	"https://schema.org/RisksOrComplicationsHealthAspect";
/// <https://schema.org/RisksOrComplicationsHealthAspect>
pub const RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_LABEL: &str = "RisksOrComplicationsHealthAspect";
pub struct RisksOrComplicationsHealthAspectIri;
impl PartialEq<&str> for RisksOrComplicationsHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_IRI_HTTP
			|| *other == RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<RisksOrComplicationsHealthAspectIri> for &str {
	fn eq(&self, other: &RisksOrComplicationsHealthAspectIri) -> bool {
		*self == RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_IRI_HTTP
			|| *self == RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct RisksOrComplicationsHealthAspectIriOrLabel;
impl PartialEq<&str> for RisksOrComplicationsHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RisksOrComplicationsHealthAspectIri
			|| *other == RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<RisksOrComplicationsHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &RisksOrComplicationsHealthAspectIriOrLabel) -> bool {
		*self == RisksOrComplicationsHealthAspectIri
			|| *self == RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_LABEL
	}
}
