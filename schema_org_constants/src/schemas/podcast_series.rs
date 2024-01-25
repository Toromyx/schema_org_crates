/// <https://schema.org/PodcastSeries>
pub const PODCAST_SERIES_IRI_HTTP: &str = "http://schema.org/PodcastSeries";
/// <https://schema.org/PodcastSeries>
pub const PODCAST_SERIES_IRI_HTTPS: &str = "https://schema.org/PodcastSeries";
/// <https://schema.org/PodcastSeries>
pub const PODCAST_SERIES_LABEL: &str = "PodcastSeries";
pub struct PodcastSeriesIri;
impl PartialEq<&str> for PodcastSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PODCAST_SERIES_IRI_HTTP || *other == PODCAST_SERIES_IRI_HTTPS
	}
}
impl PartialEq<PodcastSeriesIri> for &str {
	fn eq(&self, other: &PodcastSeriesIri) -> bool {
		*self == PODCAST_SERIES_IRI_HTTP || *self == PODCAST_SERIES_IRI_HTTPS
	}
}
pub struct PodcastSeriesIriOrLabel;
impl PartialEq<&str> for PodcastSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PodcastSeriesIri || *other == PODCAST_SERIES_LABEL
	}
}
impl PartialEq<PodcastSeriesIriOrLabel> for &str {
	fn eq(&self, other: &PodcastSeriesIriOrLabel) -> bool {
		*self == PodcastSeriesIri || *self == PODCAST_SERIES_LABEL
	}
}
