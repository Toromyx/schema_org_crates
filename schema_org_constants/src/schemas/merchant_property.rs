/// <https://schema.org/merchant>
#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
pub const MERCHANT_PROPERTY_IRI_HTTP: &str = "http://schema.org/merchant";
/// <https://schema.org/merchant>
#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
pub const MERCHANT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/merchant";
/// <https://schema.org/merchant>
#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
pub const MERCHANT_PROPERTY_LABEL: &str = "merchant";
pub struct MerchantPropertyIri;
impl PartialEq<&str> for MerchantPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_PROPERTY_IRI_HTTP || *other == MERCHANT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MerchantPropertyIri> for &str {
	fn eq(&self, other: &MerchantPropertyIri) -> bool {
		*self == MERCHANT_PROPERTY_IRI_HTTP || *self == MERCHANT_PROPERTY_IRI_HTTPS
	}
}
pub struct MerchantPropertyIriOrLabel;
impl PartialEq<&str> for MerchantPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantPropertyIri || *other == MERCHANT_PROPERTY_LABEL
	}
}
impl PartialEq<MerchantPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MerchantPropertyIriOrLabel) -> bool {
		*self == MerchantPropertyIri || *self == MERCHANT_PROPERTY_LABEL
	}
}
