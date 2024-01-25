/// <https://schema.org/RadioClip>
pub const RADIO_CLIP_IRI_HTTP: &str = "http://schema.org/RadioClip";
/// <https://schema.org/RadioClip>
pub const RADIO_CLIP_IRI_HTTPS: &str = "https://schema.org/RadioClip";
/// <https://schema.org/RadioClip>
pub const RADIO_CLIP_LABEL: &str = "RadioClip";
pub struct RadioClipIri;
impl PartialEq<&str> for RadioClipIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIO_CLIP_IRI_HTTP || *other == RADIO_CLIP_IRI_HTTPS
	}
}
impl PartialEq<RadioClipIri> for &str {
	fn eq(&self, other: &RadioClipIri) -> bool {
		*self == RADIO_CLIP_IRI_HTTP || *self == RADIO_CLIP_IRI_HTTPS
	}
}
pub struct RadioClipIriOrLabel;
impl PartialEq<&str> for RadioClipIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadioClipIri || *other == RADIO_CLIP_LABEL
	}
}
impl PartialEq<RadioClipIriOrLabel> for &str {
	fn eq(&self, other: &RadioClipIriOrLabel) -> bool {
		*self == RadioClipIri || *self == RADIO_CLIP_LABEL
	}
}
