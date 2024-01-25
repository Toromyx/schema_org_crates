/// <https://schema.org/HealthTopicContent>
pub const HEALTH_TOPIC_CONTENT_IRI_HTTP: &str = "http://schema.org/HealthTopicContent";
/// <https://schema.org/HealthTopicContent>
pub const HEALTH_TOPIC_CONTENT_IRI_HTTPS: &str = "https://schema.org/HealthTopicContent";
/// <https://schema.org/HealthTopicContent>
pub const HEALTH_TOPIC_CONTENT_LABEL: &str = "HealthTopicContent";
pub struct HealthTopicContentIri;
impl PartialEq<&str> for HealthTopicContentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_TOPIC_CONTENT_IRI_HTTP || *other == HEALTH_TOPIC_CONTENT_IRI_HTTPS
	}
}
impl PartialEq<HealthTopicContentIri> for &str {
	fn eq(&self, other: &HealthTopicContentIri) -> bool {
		*self == HEALTH_TOPIC_CONTENT_IRI_HTTP || *self == HEALTH_TOPIC_CONTENT_IRI_HTTPS
	}
}
pub struct HealthTopicContentIriOrLabel;
impl PartialEq<&str> for HealthTopicContentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthTopicContentIri || *other == HEALTH_TOPIC_CONTENT_LABEL
	}
}
impl PartialEq<HealthTopicContentIriOrLabel> for &str {
	fn eq(&self, other: &HealthTopicContentIriOrLabel) -> bool {
		*self == HealthTopicContentIri || *self == HEALTH_TOPIC_CONTENT_LABEL
	}
}
