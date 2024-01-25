/// <https://schema.org/targetCollection>
pub const TARGET_COLLECTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/targetCollection";
/// <https://schema.org/targetCollection>
pub const TARGET_COLLECTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/targetCollection";
/// <https://schema.org/targetCollection>
pub const TARGET_COLLECTION_PROPERTY_LABEL: &str = "targetCollection";
pub struct TargetCollectionPropertyIri;
impl PartialEq<&str> for TargetCollectionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TARGET_COLLECTION_PROPERTY_IRI_HTTP
			|| *other == TARGET_COLLECTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TargetCollectionPropertyIri> for &str {
	fn eq(&self, other: &TargetCollectionPropertyIri) -> bool {
		*self == TARGET_COLLECTION_PROPERTY_IRI_HTTP
			|| *self == TARGET_COLLECTION_PROPERTY_IRI_HTTPS
	}
}
pub struct TargetCollectionPropertyIriOrLabel;
impl PartialEq<&str> for TargetCollectionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TargetCollectionPropertyIri || *other == TARGET_COLLECTION_PROPERTY_LABEL
	}
}
impl PartialEq<TargetCollectionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TargetCollectionPropertyIriOrLabel) -> bool {
		*self == TargetCollectionPropertyIri || *self == TARGET_COLLECTION_PROPERTY_LABEL
	}
}
