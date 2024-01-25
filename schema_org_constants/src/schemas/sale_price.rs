/// <https://schema.org/SalePrice>
pub const SALE_PRICE_IRI_HTTP: &str = "http://schema.org/SalePrice";
/// <https://schema.org/SalePrice>
pub const SALE_PRICE_IRI_HTTPS: &str = "https://schema.org/SalePrice";
/// <https://schema.org/SalePrice>
pub const SALE_PRICE_LABEL: &str = "SalePrice";
pub struct SalePriceIri;
impl PartialEq<&str> for SalePriceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SALE_PRICE_IRI_HTTP || *other == SALE_PRICE_IRI_HTTPS
	}
}
impl PartialEq<SalePriceIri> for &str {
	fn eq(&self, other: &SalePriceIri) -> bool {
		*self == SALE_PRICE_IRI_HTTP || *self == SALE_PRICE_IRI_HTTPS
	}
}
pub struct SalePriceIriOrLabel;
impl PartialEq<&str> for SalePriceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SalePriceIri || *other == SALE_PRICE_LABEL
	}
}
impl PartialEq<SalePriceIriOrLabel> for &str {
	fn eq(&self, other: &SalePriceIriOrLabel) -> bool {
		*self == SalePriceIri || *self == SALE_PRICE_LABEL
	}
}
