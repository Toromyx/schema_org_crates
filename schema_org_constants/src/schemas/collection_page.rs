/// <https://schema.org/CollectionPage>
pub const COLLECTION_PAGE_IRI_HTTP: &str = "http://schema.org/CollectionPage";
/// <https://schema.org/CollectionPage>
pub const COLLECTION_PAGE_IRI_HTTPS: &str = "https://schema.org/CollectionPage";
/// <https://schema.org/CollectionPage>
pub const COLLECTION_PAGE_LABEL: &str = "CollectionPage";
pub struct CollectionPageIri;
impl PartialEq<&str> for CollectionPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLLECTION_PAGE_IRI_HTTP || *other == COLLECTION_PAGE_IRI_HTTPS
	}
}
impl PartialEq<CollectionPageIri> for &str {
	fn eq(&self, other: &CollectionPageIri) -> bool {
		*self == COLLECTION_PAGE_IRI_HTTP || *self == COLLECTION_PAGE_IRI_HTTPS
	}
}
pub struct CollectionPageIriOrLabel;
impl PartialEq<&str> for CollectionPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CollectionPageIri || *other == COLLECTION_PAGE_LABEL
	}
}
impl PartialEq<CollectionPageIriOrLabel> for &str {
	fn eq(&self, other: &CollectionPageIriOrLabel) -> bool {
		*self == CollectionPageIri || *self == COLLECTION_PAGE_LABEL
	}
}
