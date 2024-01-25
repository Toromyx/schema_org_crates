/// <https://schema.org/episodeNumber>
pub const EPISODE_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/episodeNumber";
/// <https://schema.org/episodeNumber>
pub const EPISODE_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/episodeNumber";
/// <https://schema.org/episodeNumber>
pub const EPISODE_NUMBER_PROPERTY_LABEL: &str = "episodeNumber";
pub struct EpisodeNumberPropertyIri;
impl PartialEq<&str> for EpisodeNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EPISODE_NUMBER_PROPERTY_IRI_HTTP || *other == EPISODE_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EpisodeNumberPropertyIri> for &str {
	fn eq(&self, other: &EpisodeNumberPropertyIri) -> bool {
		*self == EPISODE_NUMBER_PROPERTY_IRI_HTTP || *self == EPISODE_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct EpisodeNumberPropertyIriOrLabel;
impl PartialEq<&str> for EpisodeNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EpisodeNumberPropertyIri || *other == EPISODE_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<EpisodeNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EpisodeNumberPropertyIriOrLabel) -> bool {
		*self == EpisodeNumberPropertyIri || *self == EPISODE_NUMBER_PROPERTY_LABEL
	}
}
