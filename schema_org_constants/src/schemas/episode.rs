/// <https://schema.org/Episode>
pub const EPISODE_IRI_HTTP: &str = "http://schema.org/Episode";
/// <https://schema.org/Episode>
pub const EPISODE_IRI_HTTPS: &str = "https://schema.org/Episode";
/// <https://schema.org/Episode>
pub const EPISODE_LABEL: &str = "Episode";
pub struct EpisodeIri;
impl PartialEq<&str> for EpisodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EPISODE_IRI_HTTP || *other == EPISODE_IRI_HTTPS
	}
}
impl PartialEq<EpisodeIri> for &str {
	fn eq(&self, other: &EpisodeIri) -> bool {
		*self == EPISODE_IRI_HTTP || *self == EPISODE_IRI_HTTPS
	}
}
pub struct EpisodeIriOrLabel;
impl PartialEq<&str> for EpisodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EpisodeIri || *other == EPISODE_LABEL
	}
}
impl PartialEq<EpisodeIriOrLabel> for &str {
	fn eq(&self, other: &EpisodeIriOrLabel) -> bool {
		*self == EpisodeIri || *self == EPISODE_LABEL
	}
}
