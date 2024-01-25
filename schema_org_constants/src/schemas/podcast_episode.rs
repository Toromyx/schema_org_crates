/// <https://schema.org/PodcastEpisode>
pub const PODCAST_EPISODE_IRI_HTTP: &str = "http://schema.org/PodcastEpisode";
/// <https://schema.org/PodcastEpisode>
pub const PODCAST_EPISODE_IRI_HTTPS: &str = "https://schema.org/PodcastEpisode";
/// <https://schema.org/PodcastEpisode>
pub const PODCAST_EPISODE_LABEL: &str = "PodcastEpisode";
pub struct PodcastEpisodeIri;
impl PartialEq<&str> for PodcastEpisodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PODCAST_EPISODE_IRI_HTTP || *other == PODCAST_EPISODE_IRI_HTTPS
	}
}
impl PartialEq<PodcastEpisodeIri> for &str {
	fn eq(&self, other: &PodcastEpisodeIri) -> bool {
		*self == PODCAST_EPISODE_IRI_HTTP || *self == PODCAST_EPISODE_IRI_HTTPS
	}
}
pub struct PodcastEpisodeIriOrLabel;
impl PartialEq<&str> for PodcastEpisodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PodcastEpisodeIri || *other == PODCAST_EPISODE_LABEL
	}
}
impl PartialEq<PodcastEpisodeIriOrLabel> for &str {
	fn eq(&self, other: &PodcastEpisodeIriOrLabel) -> bool {
		*self == PodcastEpisodeIri || *self == PODCAST_EPISODE_LABEL
	}
}
