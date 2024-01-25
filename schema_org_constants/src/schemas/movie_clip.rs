/// <https://schema.org/MovieClip>
pub const MOVIE_CLIP_IRI_HTTP: &str = "http://schema.org/MovieClip";
/// <https://schema.org/MovieClip>
pub const MOVIE_CLIP_IRI_HTTPS: &str = "https://schema.org/MovieClip";
/// <https://schema.org/MovieClip>
pub const MOVIE_CLIP_LABEL: &str = "MovieClip";
pub struct MovieClipIri;
impl PartialEq<&str> for MovieClipIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOVIE_CLIP_IRI_HTTP || *other == MOVIE_CLIP_IRI_HTTPS
	}
}
impl PartialEq<MovieClipIri> for &str {
	fn eq(&self, other: &MovieClipIri) -> bool {
		*self == MOVIE_CLIP_IRI_HTTP || *self == MOVIE_CLIP_IRI_HTTPS
	}
}
pub struct MovieClipIriOrLabel;
impl PartialEq<&str> for MovieClipIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MovieClipIri || *other == MOVIE_CLIP_LABEL
	}
}
impl PartialEq<MovieClipIriOrLabel> for &str {
	fn eq(&self, other: &MovieClipIriOrLabel) -> bool {
		*self == MovieClipIri || *self == MOVIE_CLIP_LABEL
	}
}
