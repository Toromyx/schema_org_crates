/// <https://schema.org/contentLocation>
pub const CONTENT_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/contentLocation";
/// <https://schema.org/contentLocation>
pub const CONTENT_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contentLocation";
/// <https://schema.org/contentLocation>
pub const CONTENT_LOCATION_PROPERTY_LABEL: &str = "contentLocation";
pub struct ContentLocationPropertyIri;
impl PartialEq<&str> for ContentLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTENT_LOCATION_PROPERTY_IRI_HTTP
			|| *other == CONTENT_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContentLocationPropertyIri> for &str {
	fn eq(&self, other: &ContentLocationPropertyIri) -> bool {
		*self == CONTENT_LOCATION_PROPERTY_IRI_HTTP || *self == CONTENT_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ContentLocationPropertyIriOrLabel;
impl PartialEq<&str> for ContentLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContentLocationPropertyIri || *other == CONTENT_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<ContentLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContentLocationPropertyIriOrLabel) -> bool {
		*self == ContentLocationPropertyIri || *self == CONTENT_LOCATION_PROPERTY_LABEL
	}
}
