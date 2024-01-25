/// <https://schema.org/makesOffer>
pub const MAKES_OFFER_PROPERTY_IRI_HTTP: &str = "http://schema.org/makesOffer";
/// <https://schema.org/makesOffer>
pub const MAKES_OFFER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/makesOffer";
/// <https://schema.org/makesOffer>
pub const MAKES_OFFER_PROPERTY_LABEL: &str = "makesOffer";
pub struct MakesOfferPropertyIri;
impl PartialEq<&str> for MakesOfferPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAKES_OFFER_PROPERTY_IRI_HTTP || *other == MAKES_OFFER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MakesOfferPropertyIri> for &str {
	fn eq(&self, other: &MakesOfferPropertyIri) -> bool {
		*self == MAKES_OFFER_PROPERTY_IRI_HTTP || *self == MAKES_OFFER_PROPERTY_IRI_HTTPS
	}
}
pub struct MakesOfferPropertyIriOrLabel;
impl PartialEq<&str> for MakesOfferPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MakesOfferPropertyIri || *other == MAKES_OFFER_PROPERTY_LABEL
	}
}
impl PartialEq<MakesOfferPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MakesOfferPropertyIriOrLabel) -> bool {
		*self == MakesOfferPropertyIri || *self == MAKES_OFFER_PROPERTY_LABEL
	}
}
