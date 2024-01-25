/// <https://schema.org/MovieTheater>
pub const MOVIE_THEATER_IRI_HTTP: &str = "http://schema.org/MovieTheater";
/// <https://schema.org/MovieTheater>
pub const MOVIE_THEATER_IRI_HTTPS: &str = "https://schema.org/MovieTheater";
/// <https://schema.org/MovieTheater>
pub const MOVIE_THEATER_LABEL: &str = "MovieTheater";
pub struct MovieTheaterIri;
impl PartialEq<&str> for MovieTheaterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOVIE_THEATER_IRI_HTTP || *other == MOVIE_THEATER_IRI_HTTPS
	}
}
impl PartialEq<MovieTheaterIri> for &str {
	fn eq(&self, other: &MovieTheaterIri) -> bool {
		*self == MOVIE_THEATER_IRI_HTTP || *self == MOVIE_THEATER_IRI_HTTPS
	}
}
pub struct MovieTheaterIriOrLabel;
impl PartialEq<&str> for MovieTheaterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MovieTheaterIri || *other == MOVIE_THEATER_LABEL
	}
}
impl PartialEq<MovieTheaterIriOrLabel> for &str {
	fn eq(&self, other: &MovieTheaterIriOrLabel) -> bool {
		*self == MovieTheaterIri || *self == MOVIE_THEATER_LABEL
	}
}
