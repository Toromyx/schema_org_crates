/// <https://schema.org/acceptedOffer>
pub const ACCEPTED_OFFER_PROPERTY_IRI_HTTP: &str = "http://schema.org/acceptedOffer";
/// <https://schema.org/acceptedOffer>
pub const ACCEPTED_OFFER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/acceptedOffer";
/// <https://schema.org/acceptedOffer>
pub const ACCEPTED_OFFER_PROPERTY_LABEL: &str = "acceptedOffer";
pub struct AcceptedOfferPropertyIri;
impl PartialEq<&str> for AcceptedOfferPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCEPTED_OFFER_PROPERTY_IRI_HTTP || *other == ACCEPTED_OFFER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AcceptedOfferPropertyIri> for &str {
	fn eq(&self, other: &AcceptedOfferPropertyIri) -> bool {
		*self == ACCEPTED_OFFER_PROPERTY_IRI_HTTP || *self == ACCEPTED_OFFER_PROPERTY_IRI_HTTPS
	}
}
pub struct AcceptedOfferPropertyIriOrLabel;
impl PartialEq<&str> for AcceptedOfferPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AcceptedOfferPropertyIri || *other == ACCEPTED_OFFER_PROPERTY_LABEL
	}
}
impl PartialEq<AcceptedOfferPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AcceptedOfferPropertyIriOrLabel) -> bool {
		*self == AcceptedOfferPropertyIri || *self == ACCEPTED_OFFER_PROPERTY_LABEL
	}
}
