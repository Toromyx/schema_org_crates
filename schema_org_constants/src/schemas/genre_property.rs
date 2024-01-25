/// <https://schema.org/genre>
pub const GENRE_PROPERTY_IRI_HTTP: &str = "http://schema.org/genre";
/// <https://schema.org/genre>
pub const GENRE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/genre";
/// <https://schema.org/genre>
pub const GENRE_PROPERTY_LABEL: &str = "genre";
pub struct GenrePropertyIri;
impl PartialEq<&str> for GenrePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GENRE_PROPERTY_IRI_HTTP || *other == GENRE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GenrePropertyIri> for &str {
	fn eq(&self, other: &GenrePropertyIri) -> bool {
		*self == GENRE_PROPERTY_IRI_HTTP || *self == GENRE_PROPERTY_IRI_HTTPS
	}
}
pub struct GenrePropertyIriOrLabel;
impl PartialEq<&str> for GenrePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GenrePropertyIri || *other == GENRE_PROPERTY_LABEL
	}
}
impl PartialEq<GenrePropertyIriOrLabel> for &str {
	fn eq(&self, other: &GenrePropertyIriOrLabel) -> bool {
		*self == GenrePropertyIri || *self == GENRE_PROPERTY_LABEL
	}
}
