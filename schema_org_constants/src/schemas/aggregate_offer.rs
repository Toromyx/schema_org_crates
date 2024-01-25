/// <https://schema.org/AggregateOffer>
pub const AGGREGATE_OFFER_IRI_HTTP: &str = "http://schema.org/AggregateOffer";
/// <https://schema.org/AggregateOffer>
pub const AGGREGATE_OFFER_IRI_HTTPS: &str = "https://schema.org/AggregateOffer";
/// <https://schema.org/AggregateOffer>
pub const AGGREGATE_OFFER_LABEL: &str = "AggregateOffer";
pub struct AggregateOfferIri;
impl PartialEq<&str> for AggregateOfferIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AGGREGATE_OFFER_IRI_HTTP || *other == AGGREGATE_OFFER_IRI_HTTPS
	}
}
impl PartialEq<AggregateOfferIri> for &str {
	fn eq(&self, other: &AggregateOfferIri) -> bool {
		*self == AGGREGATE_OFFER_IRI_HTTP || *self == AGGREGATE_OFFER_IRI_HTTPS
	}
}
pub struct AggregateOfferIriOrLabel;
impl PartialEq<&str> for AggregateOfferIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AggregateOfferIri || *other == AGGREGATE_OFFER_LABEL
	}
}
impl PartialEq<AggregateOfferIriOrLabel> for &str {
	fn eq(&self, other: &AggregateOfferIriOrLabel) -> bool {
		*self == AggregateOfferIri || *self == AGGREGATE_OFFER_LABEL
	}
}
