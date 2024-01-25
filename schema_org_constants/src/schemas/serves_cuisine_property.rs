/// <https://schema.org/servesCuisine>
pub const SERVES_CUISINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/servesCuisine";
/// <https://schema.org/servesCuisine>
pub const SERVES_CUISINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/servesCuisine";
/// <https://schema.org/servesCuisine>
pub const SERVES_CUISINE_PROPERTY_LABEL: &str = "servesCuisine";
pub struct ServesCuisinePropertyIri;
impl PartialEq<&str> for ServesCuisinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVES_CUISINE_PROPERTY_IRI_HTTP || *other == SERVES_CUISINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServesCuisinePropertyIri> for &str {
	fn eq(&self, other: &ServesCuisinePropertyIri) -> bool {
		*self == SERVES_CUISINE_PROPERTY_IRI_HTTP || *self == SERVES_CUISINE_PROPERTY_IRI_HTTPS
	}
}
pub struct ServesCuisinePropertyIriOrLabel;
impl PartialEq<&str> for ServesCuisinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServesCuisinePropertyIri || *other == SERVES_CUISINE_PROPERTY_LABEL
	}
}
impl PartialEq<ServesCuisinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServesCuisinePropertyIriOrLabel) -> bool {
		*self == ServesCuisinePropertyIri || *self == SERVES_CUISINE_PROPERTY_LABEL
	}
}
