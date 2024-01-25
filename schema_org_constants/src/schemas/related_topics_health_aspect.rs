/// <https://schema.org/RelatedTopicsHealthAspect>
pub const RELATED_TOPICS_HEALTH_ASPECT_IRI_HTTP: &str =
	"http://schema.org/RelatedTopicsHealthAspect";
/// <https://schema.org/RelatedTopicsHealthAspect>
pub const RELATED_TOPICS_HEALTH_ASPECT_IRI_HTTPS: &str =
	"https://schema.org/RelatedTopicsHealthAspect";
/// <https://schema.org/RelatedTopicsHealthAspect>
pub const RELATED_TOPICS_HEALTH_ASPECT_LABEL: &str = "RelatedTopicsHealthAspect";
pub struct RelatedTopicsHealthAspectIri;
impl PartialEq<&str> for RelatedTopicsHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELATED_TOPICS_HEALTH_ASPECT_IRI_HTTP
			|| *other == RELATED_TOPICS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<RelatedTopicsHealthAspectIri> for &str {
	fn eq(&self, other: &RelatedTopicsHealthAspectIri) -> bool {
		*self == RELATED_TOPICS_HEALTH_ASPECT_IRI_HTTP
			|| *self == RELATED_TOPICS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct RelatedTopicsHealthAspectIriOrLabel;
impl PartialEq<&str> for RelatedTopicsHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelatedTopicsHealthAspectIri || *other == RELATED_TOPICS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<RelatedTopicsHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &RelatedTopicsHealthAspectIriOrLabel) -> bool {
		*self == RelatedTopicsHealthAspectIri || *self == RELATED_TOPICS_HEALTH_ASPECT_LABEL
	}
}
