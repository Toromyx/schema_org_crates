/// <https://schema.org/episode>
pub const EPISODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/episode";
/// <https://schema.org/episode>
pub const EPISODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/episode";
/// <https://schema.org/episode>
pub const EPISODE_PROPERTY_LABEL: &str = "episode";
pub struct EpisodePropertyIri;
impl PartialEq<&str> for EpisodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EPISODE_PROPERTY_IRI_HTTP || *other == EPISODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EpisodePropertyIri> for &str {
	fn eq(&self, other: &EpisodePropertyIri) -> bool {
		*self == EPISODE_PROPERTY_IRI_HTTP || *self == EPISODE_PROPERTY_IRI_HTTPS
	}
}
pub struct EpisodePropertyIriOrLabel;
impl PartialEq<&str> for EpisodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EpisodePropertyIri || *other == EPISODE_PROPERTY_LABEL
	}
}
impl PartialEq<EpisodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EpisodePropertyIriOrLabel) -> bool {
		*self == EpisodePropertyIri || *self == EPISODE_PROPERTY_LABEL
	}
}
