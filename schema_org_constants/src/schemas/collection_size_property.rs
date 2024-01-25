/// <https://schema.org/collectionSize>
pub const COLLECTION_SIZE_PROPERTY_IRI_HTTP: &str = "http://schema.org/collectionSize";
/// <https://schema.org/collectionSize>
pub const COLLECTION_SIZE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/collectionSize";
/// <https://schema.org/collectionSize>
pub const COLLECTION_SIZE_PROPERTY_LABEL: &str = "collectionSize";
pub struct CollectionSizePropertyIri;
impl PartialEq<&str> for CollectionSizePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLLECTION_SIZE_PROPERTY_IRI_HTTP || *other == COLLECTION_SIZE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CollectionSizePropertyIri> for &str {
	fn eq(&self, other: &CollectionSizePropertyIri) -> bool {
		*self == COLLECTION_SIZE_PROPERTY_IRI_HTTP || *self == COLLECTION_SIZE_PROPERTY_IRI_HTTPS
	}
}
pub struct CollectionSizePropertyIriOrLabel;
impl PartialEq<&str> for CollectionSizePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CollectionSizePropertyIri || *other == COLLECTION_SIZE_PROPERTY_LABEL
	}
}
impl PartialEq<CollectionSizePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CollectionSizePropertyIriOrLabel) -> bool {
		*self == CollectionSizePropertyIri || *self == COLLECTION_SIZE_PROPERTY_LABEL
	}
}
