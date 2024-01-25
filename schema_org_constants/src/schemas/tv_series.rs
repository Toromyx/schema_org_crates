/// <https://schema.org/TVSeries>
pub const TV_SERIES_IRI_HTTP: &str = "http://schema.org/TVSeries";
/// <https://schema.org/TVSeries>
pub const TV_SERIES_IRI_HTTPS: &str = "https://schema.org/TVSeries";
/// <https://schema.org/TVSeries>
pub const TV_SERIES_LABEL: &str = "TVSeries";
pub struct TvSeriesIri;
impl PartialEq<&str> for TvSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TV_SERIES_IRI_HTTP || *other == TV_SERIES_IRI_HTTPS
	}
}
impl PartialEq<TvSeriesIri> for &str {
	fn eq(&self, other: &TvSeriesIri) -> bool {
		*self == TV_SERIES_IRI_HTTP || *self == TV_SERIES_IRI_HTTPS
	}
}
pub struct TvSeriesIriOrLabel;
impl PartialEq<&str> for TvSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TvSeriesIri || *other == TV_SERIES_LABEL
	}
}
impl PartialEq<TvSeriesIriOrLabel> for &str {
	fn eq(&self, other: &TvSeriesIriOrLabel) -> bool {
		*self == TvSeriesIri || *self == TV_SERIES_LABEL
	}
}
