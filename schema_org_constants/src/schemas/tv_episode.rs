/// <https://schema.org/TVEpisode>
pub const TV_EPISODE_IRI_HTTP: &str = "http://schema.org/TVEpisode";
/// <https://schema.org/TVEpisode>
pub const TV_EPISODE_IRI_HTTPS: &str = "https://schema.org/TVEpisode";
/// <https://schema.org/TVEpisode>
pub const TV_EPISODE_LABEL: &str = "TVEpisode";
pub struct TvEpisodeIri;
impl PartialEq<&str> for TvEpisodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TV_EPISODE_IRI_HTTP || *other == TV_EPISODE_IRI_HTTPS
	}
}
impl PartialEq<TvEpisodeIri> for &str {
	fn eq(&self, other: &TvEpisodeIri) -> bool {
		*self == TV_EPISODE_IRI_HTTP || *self == TV_EPISODE_IRI_HTTPS
	}
}
pub struct TvEpisodeIriOrLabel;
impl PartialEq<&str> for TvEpisodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TvEpisodeIri || *other == TV_EPISODE_LABEL
	}
}
impl PartialEq<TvEpisodeIriOrLabel> for &str {
	fn eq(&self, other: &TvEpisodeIriOrLabel) -> bool {
		*self == TvEpisodeIri || *self == TV_EPISODE_LABEL
	}
}
