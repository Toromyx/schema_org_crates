/// <https://schema.org/inStoreReturnsOffered>
pub const IN_STORE_RETURNS_OFFERED_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/inStoreReturnsOffered";
/// <https://schema.org/inStoreReturnsOffered>
pub const IN_STORE_RETURNS_OFFERED_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/inStoreReturnsOffered";
/// <https://schema.org/inStoreReturnsOffered>
pub const IN_STORE_RETURNS_OFFERED_PROPERTY_LABEL: &str = "inStoreReturnsOffered";
pub struct InStoreReturnsOfferedPropertyIri;
impl PartialEq<&str> for InStoreReturnsOfferedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_STORE_RETURNS_OFFERED_PROPERTY_IRI_HTTP
			|| *other == IN_STORE_RETURNS_OFFERED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InStoreReturnsOfferedPropertyIri> for &str {
	fn eq(&self, other: &InStoreReturnsOfferedPropertyIri) -> bool {
		*self == IN_STORE_RETURNS_OFFERED_PROPERTY_IRI_HTTP
			|| *self == IN_STORE_RETURNS_OFFERED_PROPERTY_IRI_HTTPS
	}
}
pub struct InStoreReturnsOfferedPropertyIriOrLabel;
impl PartialEq<&str> for InStoreReturnsOfferedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InStoreReturnsOfferedPropertyIri
			|| *other == IN_STORE_RETURNS_OFFERED_PROPERTY_LABEL
	}
}
impl PartialEq<InStoreReturnsOfferedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InStoreReturnsOfferedPropertyIriOrLabel) -> bool {
		*self == InStoreReturnsOfferedPropertyIri
			|| *self == IN_STORE_RETURNS_OFFERED_PROPERTY_LABEL
	}
}
