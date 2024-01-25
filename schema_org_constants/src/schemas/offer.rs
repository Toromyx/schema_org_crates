/// <https://schema.org/Offer>
pub const OFFER_IRI_HTTP: &str = "http://schema.org/Offer";
/// <https://schema.org/Offer>
pub const OFFER_IRI_HTTPS: &str = "https://schema.org/Offer";
/// <https://schema.org/Offer>
pub const OFFER_LABEL: &str = "Offer";
pub struct OfferIri;
impl PartialEq<&str> for OfferIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFER_IRI_HTTP || *other == OFFER_IRI_HTTPS
	}
}
impl PartialEq<OfferIri> for &str {
	fn eq(&self, other: &OfferIri) -> bool {
		*self == OFFER_IRI_HTTP || *self == OFFER_IRI_HTTPS
	}
}
pub struct OfferIriOrLabel;
impl PartialEq<&str> for OfferIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfferIri || *other == OFFER_LABEL
	}
}
impl PartialEq<OfferIriOrLabel> for &str {
	fn eq(&self, other: &OfferIriOrLabel) -> bool {
		*self == OfferIri || *self == OFFER_LABEL
	}
}
