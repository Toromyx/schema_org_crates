/// <https://schema.org/Poster>
pub const POSTER_IRI_HTTP: &str = "http://schema.org/Poster";
/// <https://schema.org/Poster>
pub const POSTER_IRI_HTTPS: &str = "https://schema.org/Poster";
/// <https://schema.org/Poster>
pub const POSTER_LABEL: &str = "Poster";
pub struct PosterIri;
impl PartialEq<&str> for PosterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSTER_IRI_HTTP || *other == POSTER_IRI_HTTPS
	}
}
impl PartialEq<PosterIri> for &str {
	fn eq(&self, other: &PosterIri) -> bool {
		*self == POSTER_IRI_HTTP || *self == POSTER_IRI_HTTPS
	}
}
pub struct PosterIriOrLabel;
impl PartialEq<&str> for PosterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PosterIri || *other == POSTER_LABEL
	}
}
impl PartialEq<PosterIriOrLabel> for &str {
	fn eq(&self, other: &PosterIriOrLabel) -> bool {
		*self == PosterIri || *self == POSTER_LABEL
	}
}
