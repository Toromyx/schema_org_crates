/// <https://schema.org/caption>
pub const CAPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/caption";
/// <https://schema.org/caption>
pub const CAPTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/caption";
/// <https://schema.org/caption>
pub const CAPTION_PROPERTY_LABEL: &str = "caption";
pub struct CaptionPropertyIri;
impl PartialEq<&str> for CaptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CAPTION_PROPERTY_IRI_HTTP || *other == CAPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CaptionPropertyIri> for &str {
	fn eq(&self, other: &CaptionPropertyIri) -> bool {
		*self == CAPTION_PROPERTY_IRI_HTTP || *self == CAPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct CaptionPropertyIriOrLabel;
impl PartialEq<&str> for CaptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CaptionPropertyIri || *other == CAPTION_PROPERTY_LABEL
	}
}
impl PartialEq<CaptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CaptionPropertyIriOrLabel) -> bool {
		*self == CaptionPropertyIri || *self == CAPTION_PROPERTY_LABEL
	}
}
