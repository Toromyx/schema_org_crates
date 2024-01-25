/// <https://schema.org/OutOfStock>
pub const OUT_OF_STOCK_IRI_HTTP: &str = "http://schema.org/OutOfStock";
/// <https://schema.org/OutOfStock>
pub const OUT_OF_STOCK_IRI_HTTPS: &str = "https://schema.org/OutOfStock";
/// <https://schema.org/OutOfStock>
pub const OUT_OF_STOCK_LABEL: &str = "OutOfStock";
pub struct OutOfStockIri;
impl PartialEq<&str> for OutOfStockIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OUT_OF_STOCK_IRI_HTTP || *other == OUT_OF_STOCK_IRI_HTTPS
	}
}
impl PartialEq<OutOfStockIri> for &str {
	fn eq(&self, other: &OutOfStockIri) -> bool {
		*self == OUT_OF_STOCK_IRI_HTTP || *self == OUT_OF_STOCK_IRI_HTTPS
	}
}
pub struct OutOfStockIriOrLabel;
impl PartialEq<&str> for OutOfStockIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OutOfStockIri || *other == OUT_OF_STOCK_LABEL
	}
}
impl PartialEq<OutOfStockIriOrLabel> for &str {
	fn eq(&self, other: &OutOfStockIriOrLabel) -> bool {
		*self == OutOfStockIri || *self == OUT_OF_STOCK_LABEL
	}
}
