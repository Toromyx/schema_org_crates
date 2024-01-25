/// <https://schema.org/offerCount>
pub const OFFER_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/offerCount";
/// <https://schema.org/offerCount>
pub const OFFER_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/offerCount";
/// <https://schema.org/offerCount>
pub const OFFER_COUNT_PROPERTY_LABEL: &str = "offerCount";
pub struct OfferCountPropertyIri;
impl PartialEq<&str> for OfferCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFER_COUNT_PROPERTY_IRI_HTTP || *other == OFFER_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OfferCountPropertyIri> for &str {
	fn eq(&self, other: &OfferCountPropertyIri) -> bool {
		*self == OFFER_COUNT_PROPERTY_IRI_HTTP || *self == OFFER_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct OfferCountPropertyIriOrLabel;
impl PartialEq<&str> for OfferCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfferCountPropertyIri || *other == OFFER_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<OfferCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OfferCountPropertyIriOrLabel) -> bool {
		*self == OfferCountPropertyIri || *self == OFFER_COUNT_PROPERTY_LABEL
	}
}
