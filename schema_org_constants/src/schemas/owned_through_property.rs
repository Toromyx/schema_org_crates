/// <https://schema.org/ownedThrough>
pub const OWNED_THROUGH_PROPERTY_IRI_HTTP: &str = "http://schema.org/ownedThrough";
/// <https://schema.org/ownedThrough>
pub const OWNED_THROUGH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ownedThrough";
/// <https://schema.org/ownedThrough>
pub const OWNED_THROUGH_PROPERTY_LABEL: &str = "ownedThrough";
pub struct OwnedThroughPropertyIri;
impl PartialEq<&str> for OwnedThroughPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OWNED_THROUGH_PROPERTY_IRI_HTTP || *other == OWNED_THROUGH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OwnedThroughPropertyIri> for &str {
	fn eq(&self, other: &OwnedThroughPropertyIri) -> bool {
		*self == OWNED_THROUGH_PROPERTY_IRI_HTTP || *self == OWNED_THROUGH_PROPERTY_IRI_HTTPS
	}
}
pub struct OwnedThroughPropertyIriOrLabel;
impl PartialEq<&str> for OwnedThroughPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OwnedThroughPropertyIri || *other == OWNED_THROUGH_PROPERTY_LABEL
	}
}
impl PartialEq<OwnedThroughPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OwnedThroughPropertyIriOrLabel) -> bool {
		*self == OwnedThroughPropertyIri || *self == OWNED_THROUGH_PROPERTY_LABEL
	}
}
