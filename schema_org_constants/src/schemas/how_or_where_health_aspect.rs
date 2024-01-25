/// <https://schema.org/HowOrWhereHealthAspect>
pub const HOW_OR_WHERE_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/HowOrWhereHealthAspect";
/// <https://schema.org/HowOrWhereHealthAspect>
pub const HOW_OR_WHERE_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/HowOrWhereHealthAspect";
/// <https://schema.org/HowOrWhereHealthAspect>
pub const HOW_OR_WHERE_HEALTH_ASPECT_LABEL: &str = "HowOrWhereHealthAspect";
pub struct HowOrWhereHealthAspectIri;
impl PartialEq<&str> for HowOrWhereHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_OR_WHERE_HEALTH_ASPECT_IRI_HTTP
			|| *other == HOW_OR_WHERE_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<HowOrWhereHealthAspectIri> for &str {
	fn eq(&self, other: &HowOrWhereHealthAspectIri) -> bool {
		*self == HOW_OR_WHERE_HEALTH_ASPECT_IRI_HTTP
			|| *self == HOW_OR_WHERE_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct HowOrWhereHealthAspectIriOrLabel;
impl PartialEq<&str> for HowOrWhereHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowOrWhereHealthAspectIri || *other == HOW_OR_WHERE_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<HowOrWhereHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &HowOrWhereHealthAspectIriOrLabel) -> bool {
		*self == HowOrWhereHealthAspectIri || *self == HOW_OR_WHERE_HEALTH_ASPECT_LABEL
	}
}
