/// <https://schema.org/MovieRentalStore>
pub const MOVIE_RENTAL_STORE_IRI_HTTP: &str = "http://schema.org/MovieRentalStore";
/// <https://schema.org/MovieRentalStore>
pub const MOVIE_RENTAL_STORE_IRI_HTTPS: &str = "https://schema.org/MovieRentalStore";
/// <https://schema.org/MovieRentalStore>
pub const MOVIE_RENTAL_STORE_LABEL: &str = "MovieRentalStore";
pub struct MovieRentalStoreIri;
impl PartialEq<&str> for MovieRentalStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOVIE_RENTAL_STORE_IRI_HTTP || *other == MOVIE_RENTAL_STORE_IRI_HTTPS
	}
}
impl PartialEq<MovieRentalStoreIri> for &str {
	fn eq(&self, other: &MovieRentalStoreIri) -> bool {
		*self == MOVIE_RENTAL_STORE_IRI_HTTP || *self == MOVIE_RENTAL_STORE_IRI_HTTPS
	}
}
pub struct MovieRentalStoreIriOrLabel;
impl PartialEq<&str> for MovieRentalStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MovieRentalStoreIri || *other == MOVIE_RENTAL_STORE_LABEL
	}
}
impl PartialEq<MovieRentalStoreIriOrLabel> for &str {
	fn eq(&self, other: &MovieRentalStoreIriOrLabel) -> bool {
		*self == MovieRentalStoreIri || *self == MOVIE_RENTAL_STORE_LABEL
	}
}
