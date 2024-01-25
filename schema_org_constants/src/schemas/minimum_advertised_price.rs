/// <https://schema.org/MinimumAdvertisedPrice>
pub const MINIMUM_ADVERTISED_PRICE_IRI_HTTP: &str = "http://schema.org/MinimumAdvertisedPrice";
/// <https://schema.org/MinimumAdvertisedPrice>
pub const MINIMUM_ADVERTISED_PRICE_IRI_HTTPS: &str = "https://schema.org/MinimumAdvertisedPrice";
/// <https://schema.org/MinimumAdvertisedPrice>
pub const MINIMUM_ADVERTISED_PRICE_LABEL: &str = "MinimumAdvertisedPrice";
pub struct MinimumAdvertisedPriceIri;
impl PartialEq<&str> for MinimumAdvertisedPriceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MINIMUM_ADVERTISED_PRICE_IRI_HTTP || *other == MINIMUM_ADVERTISED_PRICE_IRI_HTTPS
	}
}
impl PartialEq<MinimumAdvertisedPriceIri> for &str {
	fn eq(&self, other: &MinimumAdvertisedPriceIri) -> bool {
		*self == MINIMUM_ADVERTISED_PRICE_IRI_HTTP || *self == MINIMUM_ADVERTISED_PRICE_IRI_HTTPS
	}
}
pub struct MinimumAdvertisedPriceIriOrLabel;
impl PartialEq<&str> for MinimumAdvertisedPriceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MinimumAdvertisedPriceIri || *other == MINIMUM_ADVERTISED_PRICE_LABEL
	}
}
impl PartialEq<MinimumAdvertisedPriceIriOrLabel> for &str {
	fn eq(&self, other: &MinimumAdvertisedPriceIriOrLabel) -> bool {
		*self == MinimumAdvertisedPriceIri || *self == MINIMUM_ADVERTISED_PRICE_LABEL
	}
}
