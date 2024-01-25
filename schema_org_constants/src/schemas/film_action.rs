/// <https://schema.org/FilmAction>
pub const FILM_ACTION_IRI_HTTP: &str = "http://schema.org/FilmAction";
/// <https://schema.org/FilmAction>
pub const FILM_ACTION_IRI_HTTPS: &str = "https://schema.org/FilmAction";
/// <https://schema.org/FilmAction>
pub const FILM_ACTION_LABEL: &str = "FilmAction";
pub struct FilmActionIri;
impl PartialEq<&str> for FilmActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FILM_ACTION_IRI_HTTP || *other == FILM_ACTION_IRI_HTTPS
	}
}
impl PartialEq<FilmActionIri> for &str {
	fn eq(&self, other: &FilmActionIri) -> bool {
		*self == FILM_ACTION_IRI_HTTP || *self == FILM_ACTION_IRI_HTTPS
	}
}
pub struct FilmActionIriOrLabel;
impl PartialEq<&str> for FilmActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FilmActionIri || *other == FILM_ACTION_LABEL
	}
}
impl PartialEq<FilmActionIriOrLabel> for &str {
	fn eq(&self, other: &FilmActionIriOrLabel) -> bool {
		*self == FilmActionIri || *self == FILM_ACTION_LABEL
	}
}
