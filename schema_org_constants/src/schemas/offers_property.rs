/// <https://schema.org/offers>
pub const OFFERS_PROPERTY_IRI_HTTP: &str = "http://schema.org/offers";
/// <https://schema.org/offers>
pub const OFFERS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/offers";
/// <https://schema.org/offers>
pub const OFFERS_PROPERTY_LABEL: &str = "offers";
pub struct OffersPropertyIri;
impl PartialEq<&str> for OffersPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFERS_PROPERTY_IRI_HTTP || *other == OFFERS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OffersPropertyIri> for &str {
	fn eq(&self, other: &OffersPropertyIri) -> bool {
		*self == OFFERS_PROPERTY_IRI_HTTP || *self == OFFERS_PROPERTY_IRI_HTTPS
	}
}
pub struct OffersPropertyIriOrLabel;
impl PartialEq<&str> for OffersPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OffersPropertyIri || *other == OFFERS_PROPERTY_LABEL
	}
}
impl PartialEq<OffersPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OffersPropertyIriOrLabel) -> bool {
		*self == OffersPropertyIri || *self == OFFERS_PROPERTY_LABEL
	}
}
