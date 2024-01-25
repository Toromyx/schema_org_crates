/// <https://schema.org/InStock>
pub const IN_STOCK_IRI_HTTP: &str = "http://schema.org/InStock";
/// <https://schema.org/InStock>
pub const IN_STOCK_IRI_HTTPS: &str = "https://schema.org/InStock";
/// <https://schema.org/InStock>
pub const IN_STOCK_LABEL: &str = "InStock";
pub struct InStockIri;
impl PartialEq<&str> for InStockIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_STOCK_IRI_HTTP || *other == IN_STOCK_IRI_HTTPS
	}
}
impl PartialEq<InStockIri> for &str {
	fn eq(&self, other: &InStockIri) -> bool {
		*self == IN_STOCK_IRI_HTTP || *self == IN_STOCK_IRI_HTTPS
	}
}
pub struct InStockIriOrLabel;
impl PartialEq<&str> for InStockIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InStockIri || *other == IN_STOCK_LABEL
	}
}
impl PartialEq<InStockIriOrLabel> for &str {
	fn eq(&self, other: &InStockIriOrLabel) -> bool {
		*self == InStockIri || *self == IN_STOCK_LABEL
	}
}
