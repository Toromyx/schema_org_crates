/// <https://schema.org/collection>
#[deprecated = "This schema is superseded by <https://schema.org/targetCollection>."]
pub const COLLECTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/collection";
/// <https://schema.org/collection>
#[deprecated = "This schema is superseded by <https://schema.org/targetCollection>."]
pub const COLLECTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/collection";
/// <https://schema.org/collection>
#[deprecated = "This schema is superseded by <https://schema.org/targetCollection>."]
pub const COLLECTION_PROPERTY_LABEL: &str = "collection";
pub struct CollectionPropertyIri;
impl PartialEq<&str> for CollectionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLLECTION_PROPERTY_IRI_HTTP || *other == COLLECTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CollectionPropertyIri> for &str {
	fn eq(&self, other: &CollectionPropertyIri) -> bool {
		*self == COLLECTION_PROPERTY_IRI_HTTP || *self == COLLECTION_PROPERTY_IRI_HTTPS
	}
}
pub struct CollectionPropertyIriOrLabel;
impl PartialEq<&str> for CollectionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CollectionPropertyIri || *other == COLLECTION_PROPERTY_LABEL
	}
}
impl PartialEq<CollectionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CollectionPropertyIriOrLabel) -> bool {
		*self == CollectionPropertyIri || *self == COLLECTION_PROPERTY_LABEL
	}
}
