/// <https://schema.org/PostalAddress>
pub const POSTAL_ADDRESS_IRI_HTTP: &str = "http://schema.org/PostalAddress";
/// <https://schema.org/PostalAddress>
pub const POSTAL_ADDRESS_IRI_HTTPS: &str = "https://schema.org/PostalAddress";
/// <https://schema.org/PostalAddress>
pub const POSTAL_ADDRESS_LABEL: &str = "PostalAddress";
pub struct PostalAddressIri;
impl PartialEq<&str> for PostalAddressIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSTAL_ADDRESS_IRI_HTTP || *other == POSTAL_ADDRESS_IRI_HTTPS
	}
}
impl PartialEq<PostalAddressIri> for &str {
	fn eq(&self, other: &PostalAddressIri) -> bool {
		*self == POSTAL_ADDRESS_IRI_HTTP || *self == POSTAL_ADDRESS_IRI_HTTPS
	}
}
pub struct PostalAddressIriOrLabel;
impl PartialEq<&str> for PostalAddressIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostalAddressIri || *other == POSTAL_ADDRESS_LABEL
	}
}
impl PartialEq<PostalAddressIriOrLabel> for &str {
	fn eq(&self, other: &PostalAddressIriOrLabel) -> bool {
		*self == PostalAddressIri || *self == POSTAL_ADDRESS_LABEL
	}
}
