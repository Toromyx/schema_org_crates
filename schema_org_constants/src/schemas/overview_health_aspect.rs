/// <https://schema.org/OverviewHealthAspect>
pub const OVERVIEW_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/OverviewHealthAspect";
/// <https://schema.org/OverviewHealthAspect>
pub const OVERVIEW_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/OverviewHealthAspect";
/// <https://schema.org/OverviewHealthAspect>
pub const OVERVIEW_HEALTH_ASPECT_LABEL: &str = "OverviewHealthAspect";
pub struct OverviewHealthAspectIri;
impl PartialEq<&str> for OverviewHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OVERVIEW_HEALTH_ASPECT_IRI_HTTP || *other == OVERVIEW_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<OverviewHealthAspectIri> for &str {
	fn eq(&self, other: &OverviewHealthAspectIri) -> bool {
		*self == OVERVIEW_HEALTH_ASPECT_IRI_HTTP || *self == OVERVIEW_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct OverviewHealthAspectIriOrLabel;
impl PartialEq<&str> for OverviewHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OverviewHealthAspectIri || *other == OVERVIEW_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<OverviewHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &OverviewHealthAspectIriOrLabel) -> bool {
		*self == OverviewHealthAspectIri || *self == OVERVIEW_HEALTH_ASPECT_LABEL
	}
}
