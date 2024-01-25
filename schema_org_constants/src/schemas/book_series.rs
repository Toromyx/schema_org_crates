/// <https://schema.org/BookSeries>
pub const BOOK_SERIES_IRI_HTTP: &str = "http://schema.org/BookSeries";
/// <https://schema.org/BookSeries>
pub const BOOK_SERIES_IRI_HTTPS: &str = "https://schema.org/BookSeries";
/// <https://schema.org/BookSeries>
pub const BOOK_SERIES_LABEL: &str = "BookSeries";
pub struct BookSeriesIri;
impl PartialEq<&str> for BookSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOK_SERIES_IRI_HTTP || *other == BOOK_SERIES_IRI_HTTPS
	}
}
impl PartialEq<BookSeriesIri> for &str {
	fn eq(&self, other: &BookSeriesIri) -> bool {
		*self == BOOK_SERIES_IRI_HTTP || *self == BOOK_SERIES_IRI_HTTPS
	}
}
pub struct BookSeriesIriOrLabel;
impl PartialEq<&str> for BookSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookSeriesIri || *other == BOOK_SERIES_LABEL
	}
}
impl PartialEq<BookSeriesIriOrLabel> for &str {
	fn eq(&self, other: &BookSeriesIriOrLabel) -> bool {
		*self == BookSeriesIri || *self == BOOK_SERIES_LABEL
	}
}
