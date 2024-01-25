/// <https://schema.org/sharedContent>
pub const SHARED_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/sharedContent";
/// <https://schema.org/sharedContent>
pub const SHARED_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sharedContent";
/// <https://schema.org/sharedContent>
pub const SHARED_CONTENT_PROPERTY_LABEL: &str = "sharedContent";
pub struct SharedContentPropertyIri;
impl PartialEq<&str> for SharedContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHARED_CONTENT_PROPERTY_IRI_HTTP || *other == SHARED_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SharedContentPropertyIri> for &str {
	fn eq(&self, other: &SharedContentPropertyIri) -> bool {
		*self == SHARED_CONTENT_PROPERTY_IRI_HTTP || *self == SHARED_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SharedContentPropertyIriOrLabel;
impl PartialEq<&str> for SharedContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SharedContentPropertyIri || *other == SHARED_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<SharedContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SharedContentPropertyIriOrLabel) -> bool {
		*self == SharedContentPropertyIri || *self == SHARED_CONTENT_PROPERTY_LABEL
	}
}
