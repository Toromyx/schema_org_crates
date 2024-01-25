/// <https://schema.org/ownedFrom>
pub const OWNED_FROM_PROPERTY_IRI_HTTP: &str = "http://schema.org/ownedFrom";
/// <https://schema.org/ownedFrom>
pub const OWNED_FROM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ownedFrom";
/// <https://schema.org/ownedFrom>
pub const OWNED_FROM_PROPERTY_LABEL: &str = "ownedFrom";
pub struct OwnedFromPropertyIri;
impl PartialEq<&str> for OwnedFromPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OWNED_FROM_PROPERTY_IRI_HTTP || *other == OWNED_FROM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OwnedFromPropertyIri> for &str {
	fn eq(&self, other: &OwnedFromPropertyIri) -> bool {
		*self == OWNED_FROM_PROPERTY_IRI_HTTP || *self == OWNED_FROM_PROPERTY_IRI_HTTPS
	}
}
pub struct OwnedFromPropertyIriOrLabel;
impl PartialEq<&str> for OwnedFromPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OwnedFromPropertyIri || *other == OWNED_FROM_PROPERTY_LABEL
	}
}
impl PartialEq<OwnedFromPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OwnedFromPropertyIriOrLabel) -> bool {
		*self == OwnedFromPropertyIri || *self == OWNED_FROM_PROPERTY_LABEL
	}
}
