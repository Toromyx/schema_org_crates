/// <https://schema.org/Clip>
pub const CLIP_IRI_HTTP: &str = "http://schema.org/Clip";
/// <https://schema.org/Clip>
pub const CLIP_IRI_HTTPS: &str = "https://schema.org/Clip";
/// <https://schema.org/Clip>
pub const CLIP_LABEL: &str = "Clip";
pub struct ClipIri;
impl PartialEq<&str> for ClipIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLIP_IRI_HTTP || *other == CLIP_IRI_HTTPS
	}
}
impl PartialEq<ClipIri> for &str {
	fn eq(&self, other: &ClipIri) -> bool {
		*self == CLIP_IRI_HTTP || *self == CLIP_IRI_HTTPS
	}
}
pub struct ClipIriOrLabel;
impl PartialEq<&str> for ClipIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClipIri || *other == CLIP_LABEL
	}
}
impl PartialEq<ClipIriOrLabel> for &str {
	fn eq(&self, other: &ClipIriOrLabel) -> bool {
		*self == ClipIri || *self == CLIP_LABEL
	}
}
