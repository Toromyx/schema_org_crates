/// <https://schema.org/creator>
pub const CREATOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/creator";
/// <https://schema.org/creator>
pub const CREATOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/creator";
/// <https://schema.org/creator>
pub const CREATOR_PROPERTY_LABEL: &str = "creator";
pub struct CreatorPropertyIri;
impl PartialEq<&str> for CreatorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREATOR_PROPERTY_IRI_HTTP || *other == CREATOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CreatorPropertyIri> for &str {
	fn eq(&self, other: &CreatorPropertyIri) -> bool {
		*self == CREATOR_PROPERTY_IRI_HTTP || *self == CREATOR_PROPERTY_IRI_HTTPS
	}
}
pub struct CreatorPropertyIriOrLabel;
impl PartialEq<&str> for CreatorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreatorPropertyIri || *other == CREATOR_PROPERTY_LABEL
	}
}
impl PartialEq<CreatorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CreatorPropertyIriOrLabel) -> bool {
		*self == CreatorPropertyIri || *self == CREATOR_PROPERTY_LABEL
	}
}
