/// <https://schema.org/RadioEpisode>
pub const RADIO_EPISODE_IRI_HTTP: &str = "http://schema.org/RadioEpisode";
/// <https://schema.org/RadioEpisode>
pub const RADIO_EPISODE_IRI_HTTPS: &str = "https://schema.org/RadioEpisode";
/// <https://schema.org/RadioEpisode>
pub const RADIO_EPISODE_LABEL: &str = "RadioEpisode";
pub struct RadioEpisodeIri;
impl PartialEq<&str> for RadioEpisodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIO_EPISODE_IRI_HTTP || *other == RADIO_EPISODE_IRI_HTTPS
	}
}
impl PartialEq<RadioEpisodeIri> for &str {
	fn eq(&self, other: &RadioEpisodeIri) -> bool {
		*self == RADIO_EPISODE_IRI_HTTP || *self == RADIO_EPISODE_IRI_HTTPS
	}
}
pub struct RadioEpisodeIriOrLabel;
impl PartialEq<&str> for RadioEpisodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadioEpisodeIri || *other == RADIO_EPISODE_LABEL
	}
}
impl PartialEq<RadioEpisodeIriOrLabel> for &str {
	fn eq(&self, other: &RadioEpisodeIriOrLabel) -> bool {
		*self == RadioEpisodeIri || *self == RADIO_EPISODE_LABEL
	}
}
