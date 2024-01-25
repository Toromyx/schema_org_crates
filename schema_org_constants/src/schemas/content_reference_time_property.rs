/// <https://schema.org/contentReferenceTime>
pub const CONTENT_REFERENCE_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/contentReferenceTime";
/// <https://schema.org/contentReferenceTime>
pub const CONTENT_REFERENCE_TIME_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/contentReferenceTime";
/// <https://schema.org/contentReferenceTime>
pub const CONTENT_REFERENCE_TIME_PROPERTY_LABEL: &str = "contentReferenceTime";
pub struct ContentReferenceTimePropertyIri;
impl PartialEq<&str> for ContentReferenceTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTENT_REFERENCE_TIME_PROPERTY_IRI_HTTP
			|| *other == CONTENT_REFERENCE_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContentReferenceTimePropertyIri> for &str {
	fn eq(&self, other: &ContentReferenceTimePropertyIri) -> bool {
		*self == CONTENT_REFERENCE_TIME_PROPERTY_IRI_HTTP
			|| *self == CONTENT_REFERENCE_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct ContentReferenceTimePropertyIriOrLabel;
impl PartialEq<&str> for ContentReferenceTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContentReferenceTimePropertyIri || *other == CONTENT_REFERENCE_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<ContentReferenceTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContentReferenceTimePropertyIriOrLabel) -> bool {
		*self == ContentReferenceTimePropertyIri || *self == CONTENT_REFERENCE_TIME_PROPERTY_LABEL
	}
}
