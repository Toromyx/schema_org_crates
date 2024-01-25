/// <https://schema.org/MovieSeries>
pub const MOVIE_SERIES_IRI_HTTP: &str = "http://schema.org/MovieSeries";
/// <https://schema.org/MovieSeries>
pub const MOVIE_SERIES_IRI_HTTPS: &str = "https://schema.org/MovieSeries";
/// <https://schema.org/MovieSeries>
pub const MOVIE_SERIES_LABEL: &str = "MovieSeries";
pub struct MovieSeriesIri;
impl PartialEq<&str> for MovieSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOVIE_SERIES_IRI_HTTP || *other == MOVIE_SERIES_IRI_HTTPS
	}
}
impl PartialEq<MovieSeriesIri> for &str {
	fn eq(&self, other: &MovieSeriesIri) -> bool {
		*self == MOVIE_SERIES_IRI_HTTP || *self == MOVIE_SERIES_IRI_HTTPS
	}
}
pub struct MovieSeriesIriOrLabel;
impl PartialEq<&str> for MovieSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MovieSeriesIri || *other == MOVIE_SERIES_LABEL
	}
}
impl PartialEq<MovieSeriesIriOrLabel> for &str {
	fn eq(&self, other: &MovieSeriesIriOrLabel) -> bool {
		*self == MovieSeriesIri || *self == MOVIE_SERIES_LABEL
	}
}
