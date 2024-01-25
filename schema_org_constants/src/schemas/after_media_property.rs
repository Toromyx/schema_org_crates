/// <https://schema.org/afterMedia>
pub const AFTER_MEDIA_PROPERTY_IRI_HTTP: &str = "http://schema.org/afterMedia";
/// <https://schema.org/afterMedia>
pub const AFTER_MEDIA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/afterMedia";
/// <https://schema.org/afterMedia>
pub const AFTER_MEDIA_PROPERTY_LABEL: &str = "afterMedia";
pub struct AfterMediaPropertyIri;
impl PartialEq<&str> for AfterMediaPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AFTER_MEDIA_PROPERTY_IRI_HTTP || *other == AFTER_MEDIA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AfterMediaPropertyIri> for &str {
	fn eq(&self, other: &AfterMediaPropertyIri) -> bool {
		*self == AFTER_MEDIA_PROPERTY_IRI_HTTP || *self == AFTER_MEDIA_PROPERTY_IRI_HTTPS
	}
}
pub struct AfterMediaPropertyIriOrLabel;
impl PartialEq<&str> for AfterMediaPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AfterMediaPropertyIri || *other == AFTER_MEDIA_PROPERTY_LABEL
	}
}
impl PartialEq<AfterMediaPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AfterMediaPropertyIriOrLabel) -> bool {
		*self == AfterMediaPropertyIri || *self == AFTER_MEDIA_PROPERTY_LABEL
	}
}
