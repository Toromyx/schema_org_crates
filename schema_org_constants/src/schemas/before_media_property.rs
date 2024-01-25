/// <https://schema.org/beforeMedia>
pub const BEFORE_MEDIA_PROPERTY_IRI_HTTP: &str = "http://schema.org/beforeMedia";
/// <https://schema.org/beforeMedia>
pub const BEFORE_MEDIA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/beforeMedia";
/// <https://schema.org/beforeMedia>
pub const BEFORE_MEDIA_PROPERTY_LABEL: &str = "beforeMedia";
pub struct BeforeMediaPropertyIri;
impl PartialEq<&str> for BeforeMediaPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BEFORE_MEDIA_PROPERTY_IRI_HTTP || *other == BEFORE_MEDIA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BeforeMediaPropertyIri> for &str {
	fn eq(&self, other: &BeforeMediaPropertyIri) -> bool {
		*self == BEFORE_MEDIA_PROPERTY_IRI_HTTP || *self == BEFORE_MEDIA_PROPERTY_IRI_HTTPS
	}
}
pub struct BeforeMediaPropertyIriOrLabel;
impl PartialEq<&str> for BeforeMediaPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BeforeMediaPropertyIri || *other == BEFORE_MEDIA_PROPERTY_LABEL
	}
}
impl PartialEq<BeforeMediaPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BeforeMediaPropertyIriOrLabel) -> bool {
		*self == BeforeMediaPropertyIri || *self == BEFORE_MEDIA_PROPERTY_LABEL
	}
}
