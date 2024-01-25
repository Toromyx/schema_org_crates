/// <https://schema.org/Movie>
pub const MOVIE_IRI_HTTP: &str = "http://schema.org/Movie";
/// <https://schema.org/Movie>
pub const MOVIE_IRI_HTTPS: &str = "https://schema.org/Movie";
/// <https://schema.org/Movie>
pub const MOVIE_LABEL: &str = "Movie";
pub struct MovieIri;
impl PartialEq<&str> for MovieIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOVIE_IRI_HTTP || *other == MOVIE_IRI_HTTPS
	}
}
impl PartialEq<MovieIri> for &str {
	fn eq(&self, other: &MovieIri) -> bool {
		*self == MOVIE_IRI_HTTP || *self == MOVIE_IRI_HTTPS
	}
}
pub struct MovieIriOrLabel;
impl PartialEq<&str> for MovieIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MovieIri || *other == MOVIE_LABEL
	}
}
impl PartialEq<MovieIriOrLabel> for &str {
	fn eq(&self, other: &MovieIriOrLabel) -> bool {
		*self == MovieIri || *self == MOVIE_LABEL
	}
}
