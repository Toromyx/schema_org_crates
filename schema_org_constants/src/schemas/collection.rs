/// <https://schema.org/Collection>
pub const COLLECTION_IRI_HTTP: &str = "http://schema.org/Collection";
/// <https://schema.org/Collection>
pub const COLLECTION_IRI_HTTPS: &str = "https://schema.org/Collection";
/// <https://schema.org/Collection>
pub const COLLECTION_LABEL: &str = "Collection";
pub struct CollectionIri;
impl PartialEq<&str> for CollectionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLLECTION_IRI_HTTP || *other == COLLECTION_IRI_HTTPS
	}
}
impl PartialEq<CollectionIri> for &str {
	fn eq(&self, other: &CollectionIri) -> bool {
		*self == COLLECTION_IRI_HTTP || *self == COLLECTION_IRI_HTTPS
	}
}
pub struct CollectionIriOrLabel;
impl PartialEq<&str> for CollectionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CollectionIri || *other == COLLECTION_LABEL
	}
}
impl PartialEq<CollectionIriOrLabel> for &str {
	fn eq(&self, other: &CollectionIriOrLabel) -> bool {
		*self == CollectionIri || *self == COLLECTION_LABEL
	}
}
