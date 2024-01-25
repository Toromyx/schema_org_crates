/// <https://schema.org/OfferItemCondition>
pub const OFFER_ITEM_CONDITION_IRI_HTTP: &str = "http://schema.org/OfferItemCondition";
/// <https://schema.org/OfferItemCondition>
pub const OFFER_ITEM_CONDITION_IRI_HTTPS: &str = "https://schema.org/OfferItemCondition";
/// <https://schema.org/OfferItemCondition>
pub const OFFER_ITEM_CONDITION_LABEL: &str = "OfferItemCondition";
pub struct OfferItemConditionIri;
impl PartialEq<&str> for OfferItemConditionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFER_ITEM_CONDITION_IRI_HTTP || *other == OFFER_ITEM_CONDITION_IRI_HTTPS
	}
}
impl PartialEq<OfferItemConditionIri> for &str {
	fn eq(&self, other: &OfferItemConditionIri) -> bool {
		*self == OFFER_ITEM_CONDITION_IRI_HTTP || *self == OFFER_ITEM_CONDITION_IRI_HTTPS
	}
}
pub struct OfferItemConditionIriOrLabel;
impl PartialEq<&str> for OfferItemConditionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfferItemConditionIri || *other == OFFER_ITEM_CONDITION_LABEL
	}
}
impl PartialEq<OfferItemConditionIriOrLabel> for &str {
	fn eq(&self, other: &OfferItemConditionIriOrLabel) -> bool {
		*self == OfferItemConditionIri || *self == OFFER_ITEM_CONDITION_LABEL
	}
}
