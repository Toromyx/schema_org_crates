/// <https://schema.org/seller>
pub const SELLER_PROPERTY_IRI_HTTP: &str = "http://schema.org/seller";
/// <https://schema.org/seller>
pub const SELLER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seller";
/// <https://schema.org/seller>
pub const SELLER_PROPERTY_LABEL: &str = "seller";
pub struct SellerPropertyIri;
impl PartialEq<&str> for SellerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SELLER_PROPERTY_IRI_HTTP || *other == SELLER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SellerPropertyIri> for &str {
	fn eq(&self, other: &SellerPropertyIri) -> bool {
		*self == SELLER_PROPERTY_IRI_HTTP || *self == SELLER_PROPERTY_IRI_HTTPS
	}
}
pub struct SellerPropertyIriOrLabel;
impl PartialEq<&str> for SellerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SellerPropertyIri || *other == SELLER_PROPERTY_LABEL
	}
}
impl PartialEq<SellerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SellerPropertyIriOrLabel) -> bool {
		*self == SellerPropertyIri || *self == SELLER_PROPERTY_LABEL
	}
}
