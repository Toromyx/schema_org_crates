/// <https://schema.org/lyrics>
pub const LYRICS_PROPERTY_IRI_HTTP: &str = "http://schema.org/lyrics";
/// <https://schema.org/lyrics>
pub const LYRICS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/lyrics";
/// <https://schema.org/lyrics>
pub const LYRICS_PROPERTY_LABEL: &str = "lyrics";
pub struct LyricsPropertyIri;
impl PartialEq<&str> for LyricsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LYRICS_PROPERTY_IRI_HTTP || *other == LYRICS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LyricsPropertyIri> for &str {
	fn eq(&self, other: &LyricsPropertyIri) -> bool {
		*self == LYRICS_PROPERTY_IRI_HTTP || *self == LYRICS_PROPERTY_IRI_HTTPS
	}
}
pub struct LyricsPropertyIriOrLabel;
impl PartialEq<&str> for LyricsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LyricsPropertyIri || *other == LYRICS_PROPERTY_LABEL
	}
}
impl PartialEq<LyricsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LyricsPropertyIriOrLabel) -> bool {
		*self == LyricsPropertyIri || *self == LYRICS_PROPERTY_LABEL
	}
}
