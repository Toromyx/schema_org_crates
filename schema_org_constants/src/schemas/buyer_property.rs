/// <https://schema.org/buyer>
pub const BUYER_PROPERTY_IRI_HTTP: &str = "http://schema.org/buyer";
/// <https://schema.org/buyer>
pub const BUYER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/buyer";
/// <https://schema.org/buyer>
pub const BUYER_PROPERTY_LABEL: &str = "buyer";
pub struct BuyerPropertyIri;
impl PartialEq<&str> for BuyerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUYER_PROPERTY_IRI_HTTP || *other == BUYER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BuyerPropertyIri> for &str {
	fn eq(&self, other: &BuyerPropertyIri) -> bool {
		*self == BUYER_PROPERTY_IRI_HTTP || *self == BUYER_PROPERTY_IRI_HTTPS
	}
}
pub struct BuyerPropertyIriOrLabel;
impl PartialEq<&str> for BuyerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BuyerPropertyIri || *other == BUYER_PROPERTY_LABEL
	}
}
impl PartialEq<BuyerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BuyerPropertyIriOrLabel) -> bool {
		*self == BuyerPropertyIri || *self == BUYER_PROPERTY_LABEL
	}
}
