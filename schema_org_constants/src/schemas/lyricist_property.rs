/// <https://schema.org/lyricist>
pub const LYRICIST_PROPERTY_IRI_HTTP: &str = "http://schema.org/lyricist";
/// <https://schema.org/lyricist>
pub const LYRICIST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/lyricist";
/// <https://schema.org/lyricist>
pub const LYRICIST_PROPERTY_LABEL: &str = "lyricist";
pub struct LyricistPropertyIri;
impl PartialEq<&str> for LyricistPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LYRICIST_PROPERTY_IRI_HTTP || *other == LYRICIST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LyricistPropertyIri> for &str {
	fn eq(&self, other: &LyricistPropertyIri) -> bool {
		*self == LYRICIST_PROPERTY_IRI_HTTP || *self == LYRICIST_PROPERTY_IRI_HTTPS
	}
}
pub struct LyricistPropertyIriOrLabel;
impl PartialEq<&str> for LyricistPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LyricistPropertyIri || *other == LYRICIST_PROPERTY_LABEL
	}
}
impl PartialEq<LyricistPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LyricistPropertyIriOrLabel) -> bool {
		*self == LyricistPropertyIri || *self == LYRICIST_PROPERTY_LABEL
	}
}
