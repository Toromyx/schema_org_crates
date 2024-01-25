/// <https://schema.org/merchantReturnLink>
pub const MERCHANT_RETURN_LINK_PROPERTY_IRI_HTTP: &str = "http://schema.org/merchantReturnLink";
/// <https://schema.org/merchantReturnLink>
pub const MERCHANT_RETURN_LINK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/merchantReturnLink";
/// <https://schema.org/merchantReturnLink>
pub const MERCHANT_RETURN_LINK_PROPERTY_LABEL: &str = "merchantReturnLink";
pub struct MerchantReturnLinkPropertyIri;
impl PartialEq<&str> for MerchantReturnLinkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_LINK_PROPERTY_IRI_HTTP
			|| *other == MERCHANT_RETURN_LINK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnLinkPropertyIri> for &str {
	fn eq(&self, other: &MerchantReturnLinkPropertyIri) -> bool {
		*self == MERCHANT_RETURN_LINK_PROPERTY_IRI_HTTP
			|| *self == MERCHANT_RETURN_LINK_PROPERTY_IRI_HTTPS
	}
}
pub struct MerchantReturnLinkPropertyIriOrLabel;
impl PartialEq<&str> for MerchantReturnLinkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnLinkPropertyIri || *other == MERCHANT_RETURN_LINK_PROPERTY_LABEL
	}
}
impl PartialEq<MerchantReturnLinkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnLinkPropertyIriOrLabel) -> bool {
		*self == MerchantReturnLinkPropertyIri || *self == MERCHANT_RETURN_LINK_PROPERTY_LABEL
	}
}
