/// <https://schema.org/VideoGameSeries>
pub const VIDEO_GAME_SERIES_IRI_HTTP: &str = "http://schema.org/VideoGameSeries";
/// <https://schema.org/VideoGameSeries>
pub const VIDEO_GAME_SERIES_IRI_HTTPS: &str = "https://schema.org/VideoGameSeries";
/// <https://schema.org/VideoGameSeries>
pub const VIDEO_GAME_SERIES_LABEL: &str = "VideoGameSeries";
pub struct VideoGameSeriesIri;
impl PartialEq<&str> for VideoGameSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_GAME_SERIES_IRI_HTTP || *other == VIDEO_GAME_SERIES_IRI_HTTPS
	}
}
impl PartialEq<VideoGameSeriesIri> for &str {
	fn eq(&self, other: &VideoGameSeriesIri) -> bool {
		*self == VIDEO_GAME_SERIES_IRI_HTTP || *self == VIDEO_GAME_SERIES_IRI_HTTPS
	}
}
pub struct VideoGameSeriesIriOrLabel;
impl PartialEq<&str> for VideoGameSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoGameSeriesIri || *other == VIDEO_GAME_SERIES_LABEL
	}
}
impl PartialEq<VideoGameSeriesIriOrLabel> for &str {
	fn eq(&self, other: &VideoGameSeriesIriOrLabel) -> bool {
		*self == VideoGameSeriesIri || *self == VIDEO_GAME_SERIES_LABEL
	}
}
