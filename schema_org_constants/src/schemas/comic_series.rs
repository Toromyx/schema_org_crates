/// <https://schema.org/ComicSeries>
pub const COMIC_SERIES_IRI_HTTP: &str = "http://schema.org/ComicSeries";
/// <https://schema.org/ComicSeries>
pub const COMIC_SERIES_IRI_HTTPS: &str = "https://schema.org/ComicSeries";
/// <https://schema.org/ComicSeries>
pub const COMIC_SERIES_LABEL: &str = "ComicSeries";
pub struct ComicSeriesIri;
impl PartialEq<&str> for ComicSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMIC_SERIES_IRI_HTTP || *other == COMIC_SERIES_IRI_HTTPS
	}
}
impl PartialEq<ComicSeriesIri> for &str {
	fn eq(&self, other: &ComicSeriesIri) -> bool {
		*self == COMIC_SERIES_IRI_HTTP || *self == COMIC_SERIES_IRI_HTTPS
	}
}
pub struct ComicSeriesIriOrLabel;
impl PartialEq<&str> for ComicSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComicSeriesIri || *other == COMIC_SERIES_LABEL
	}
}
impl PartialEq<ComicSeriesIriOrLabel> for &str {
	fn eq(&self, other: &ComicSeriesIriOrLabel) -> bool {
		*self == ComicSeriesIri || *self == COMIC_SERIES_LABEL
	}
}
