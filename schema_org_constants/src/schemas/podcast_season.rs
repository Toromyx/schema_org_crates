/// <https://schema.org/PodcastSeason>
pub const PODCAST_SEASON_IRI_HTTP: &str = "http://schema.org/PodcastSeason";
/// <https://schema.org/PodcastSeason>
pub const PODCAST_SEASON_IRI_HTTPS: &str = "https://schema.org/PodcastSeason";
/// <https://schema.org/PodcastSeason>
pub const PODCAST_SEASON_LABEL: &str = "PodcastSeason";
pub struct PodcastSeasonIri;
impl PartialEq<&str> for PodcastSeasonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PODCAST_SEASON_IRI_HTTP || *other == PODCAST_SEASON_IRI_HTTPS
	}
}
impl PartialEq<PodcastSeasonIri> for &str {
	fn eq(&self, other: &PodcastSeasonIri) -> bool {
		*self == PODCAST_SEASON_IRI_HTTP || *self == PODCAST_SEASON_IRI_HTTPS
	}
}
pub struct PodcastSeasonIriOrLabel;
impl PartialEq<&str> for PodcastSeasonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PodcastSeasonIri || *other == PODCAST_SEASON_LABEL
	}
}
impl PartialEq<PodcastSeasonIriOrLabel> for &str {
	fn eq(&self, other: &PodcastSeasonIriOrLabel) -> bool {
		*self == PodcastSeasonIri || *self == PODCAST_SEASON_LABEL
	}
}
