/// <https://schema.org/OfferShippingDetails>
pub const OFFER_SHIPPING_DETAILS_IRI_HTTP: &str = "http://schema.org/OfferShippingDetails";
/// <https://schema.org/OfferShippingDetails>
pub const OFFER_SHIPPING_DETAILS_IRI_HTTPS: &str = "https://schema.org/OfferShippingDetails";
/// <https://schema.org/OfferShippingDetails>
pub const OFFER_SHIPPING_DETAILS_LABEL: &str = "OfferShippingDetails";
pub struct OfferShippingDetailsIri;
impl PartialEq<&str> for OfferShippingDetailsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFER_SHIPPING_DETAILS_IRI_HTTP || *other == OFFER_SHIPPING_DETAILS_IRI_HTTPS
	}
}
impl PartialEq<OfferShippingDetailsIri> for &str {
	fn eq(&self, other: &OfferShippingDetailsIri) -> bool {
		*self == OFFER_SHIPPING_DETAILS_IRI_HTTP || *self == OFFER_SHIPPING_DETAILS_IRI_HTTPS
	}
}
pub struct OfferShippingDetailsIriOrLabel;
impl PartialEq<&str> for OfferShippingDetailsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfferShippingDetailsIri || *other == OFFER_SHIPPING_DETAILS_LABEL
	}
}
impl PartialEq<OfferShippingDetailsIriOrLabel> for &str {
	fn eq(&self, other: &OfferShippingDetailsIriOrLabel) -> bool {
		*self == OfferShippingDetailsIri || *self == OFFER_SHIPPING_DETAILS_LABEL
	}
}
