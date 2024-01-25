/// <https://schema.org/OfferForPurchase>
pub const OFFER_FOR_PURCHASE_IRI_HTTP: &str = "http://schema.org/OfferForPurchase";
/// <https://schema.org/OfferForPurchase>
pub const OFFER_FOR_PURCHASE_IRI_HTTPS: &str = "https://schema.org/OfferForPurchase";
/// <https://schema.org/OfferForPurchase>
pub const OFFER_FOR_PURCHASE_LABEL: &str = "OfferForPurchase";
pub struct OfferForPurchaseIri;
impl PartialEq<&str> for OfferForPurchaseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFER_FOR_PURCHASE_IRI_HTTP || *other == OFFER_FOR_PURCHASE_IRI_HTTPS
	}
}
impl PartialEq<OfferForPurchaseIri> for &str {
	fn eq(&self, other: &OfferForPurchaseIri) -> bool {
		*self == OFFER_FOR_PURCHASE_IRI_HTTP || *self == OFFER_FOR_PURCHASE_IRI_HTTPS
	}
}
pub struct OfferForPurchaseIriOrLabel;
impl PartialEq<&str> for OfferForPurchaseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfferForPurchaseIri || *other == OFFER_FOR_PURCHASE_LABEL
	}
}
impl PartialEq<OfferForPurchaseIriOrLabel> for &str {
	fn eq(&self, other: &OfferForPurchaseIriOrLabel) -> bool {
		*self == OfferForPurchaseIri || *self == OFFER_FOR_PURCHASE_LABEL
	}
}
