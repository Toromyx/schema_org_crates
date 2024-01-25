/// <https://schema.org/TVClip>
pub const TV_CLIP_IRI_HTTP: &str = "http://schema.org/TVClip";
/// <https://schema.org/TVClip>
pub const TV_CLIP_IRI_HTTPS: &str = "https://schema.org/TVClip";
/// <https://schema.org/TVClip>
pub const TV_CLIP_LABEL: &str = "TVClip";
pub struct TvClipIri;
impl PartialEq<&str> for TvClipIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TV_CLIP_IRI_HTTP || *other == TV_CLIP_IRI_HTTPS
	}
}
impl PartialEq<TvClipIri> for &str {
	fn eq(&self, other: &TvClipIri) -> bool {
		*self == TV_CLIP_IRI_HTTP || *self == TV_CLIP_IRI_HTTPS
	}
}
pub struct TvClipIriOrLabel;
impl PartialEq<&str> for TvClipIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TvClipIri || *other == TV_CLIP_LABEL
	}
}
impl PartialEq<TvClipIriOrLabel> for &str {
	fn eq(&self, other: &TvClipIriOrLabel) -> bool {
		*self == TvClipIri || *self == TV_CLIP_LABEL
	}
}
