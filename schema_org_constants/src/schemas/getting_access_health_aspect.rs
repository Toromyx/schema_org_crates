/// <https://schema.org/GettingAccessHealthAspect>
pub const GETTING_ACCESS_HEALTH_ASPECT_IRI_HTTP: &str =
	"http://schema.org/GettingAccessHealthAspect";
/// <https://schema.org/GettingAccessHealthAspect>
pub const GETTING_ACCESS_HEALTH_ASPECT_IRI_HTTPS: &str =
	"https://schema.org/GettingAccessHealthAspect";
/// <https://schema.org/GettingAccessHealthAspect>
pub const GETTING_ACCESS_HEALTH_ASPECT_LABEL: &str = "GettingAccessHealthAspect";
pub struct GettingAccessHealthAspectIri;
impl PartialEq<&str> for GettingAccessHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GETTING_ACCESS_HEALTH_ASPECT_IRI_HTTP
			|| *other == GETTING_ACCESS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<GettingAccessHealthAspectIri> for &str {
	fn eq(&self, other: &GettingAccessHealthAspectIri) -> bool {
		*self == GETTING_ACCESS_HEALTH_ASPECT_IRI_HTTP
			|| *self == GETTING_ACCESS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct GettingAccessHealthAspectIriOrLabel;
impl PartialEq<&str> for GettingAccessHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GettingAccessHealthAspectIri || *other == GETTING_ACCESS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<GettingAccessHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &GettingAccessHealthAspectIriOrLabel) -> bool {
		*self == GettingAccessHealthAspectIri || *self == GETTING_ACCESS_HEALTH_ASPECT_LABEL
	}
}
