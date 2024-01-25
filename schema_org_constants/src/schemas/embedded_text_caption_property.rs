/// <https://schema.org/embeddedTextCaption>
pub const EMBEDDED_TEXT_CAPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/embeddedTextCaption";
/// <https://schema.org/embeddedTextCaption>
pub const EMBEDDED_TEXT_CAPTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/embeddedTextCaption";
/// <https://schema.org/embeddedTextCaption>
pub const EMBEDDED_TEXT_CAPTION_PROPERTY_LABEL: &str = "embeddedTextCaption";
pub struct EmbeddedTextCaptionPropertyIri;
impl PartialEq<&str> for EmbeddedTextCaptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMBEDDED_TEXT_CAPTION_PROPERTY_IRI_HTTP
			|| *other == EMBEDDED_TEXT_CAPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmbeddedTextCaptionPropertyIri> for &str {
	fn eq(&self, other: &EmbeddedTextCaptionPropertyIri) -> bool {
		*self == EMBEDDED_TEXT_CAPTION_PROPERTY_IRI_HTTP
			|| *self == EMBEDDED_TEXT_CAPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct EmbeddedTextCaptionPropertyIriOrLabel;
impl PartialEq<&str> for EmbeddedTextCaptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmbeddedTextCaptionPropertyIri || *other == EMBEDDED_TEXT_CAPTION_PROPERTY_LABEL
	}
}
impl PartialEq<EmbeddedTextCaptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmbeddedTextCaptionPropertyIriOrLabel) -> bool {
		*self == EmbeddedTextCaptionPropertyIri || *self == EMBEDDED_TEXT_CAPTION_PROPERTY_LABEL
	}
}
