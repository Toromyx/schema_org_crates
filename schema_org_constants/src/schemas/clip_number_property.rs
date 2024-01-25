/// <https://schema.org/clipNumber>
pub const CLIP_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/clipNumber";
/// <https://schema.org/clipNumber>
pub const CLIP_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/clipNumber";
/// <https://schema.org/clipNumber>
pub const CLIP_NUMBER_PROPERTY_LABEL: &str = "clipNumber";
pub struct ClipNumberPropertyIri;
impl PartialEq<&str> for ClipNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLIP_NUMBER_PROPERTY_IRI_HTTP || *other == CLIP_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ClipNumberPropertyIri> for &str {
	fn eq(&self, other: &ClipNumberPropertyIri) -> bool {
		*self == CLIP_NUMBER_PROPERTY_IRI_HTTP || *self == CLIP_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct ClipNumberPropertyIriOrLabel;
impl PartialEq<&str> for ClipNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClipNumberPropertyIri || *other == CLIP_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<ClipNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ClipNumberPropertyIriOrLabel) -> bool {
		*self == ClipNumberPropertyIri || *self == CLIP_NUMBER_PROPERTY_LABEL
	}
}
