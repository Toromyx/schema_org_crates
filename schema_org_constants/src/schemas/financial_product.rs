/// <https://schema.org/FinancialProduct>
pub const FINANCIAL_PRODUCT_IRI_HTTP: &str = "http://schema.org/FinancialProduct";
/// <https://schema.org/FinancialProduct>
pub const FINANCIAL_PRODUCT_IRI_HTTPS: &str = "https://schema.org/FinancialProduct";
/// <https://schema.org/FinancialProduct>
pub const FINANCIAL_PRODUCT_LABEL: &str = "FinancialProduct";
pub struct FinancialProductIri;
impl PartialEq<&str> for FinancialProductIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FINANCIAL_PRODUCT_IRI_HTTP || *other == FINANCIAL_PRODUCT_IRI_HTTPS
	}
}
impl PartialEq<FinancialProductIri> for &str {
	fn eq(&self, other: &FinancialProductIri) -> bool {
		*self == FINANCIAL_PRODUCT_IRI_HTTP || *self == FINANCIAL_PRODUCT_IRI_HTTPS
	}
}
pub struct FinancialProductIriOrLabel;
impl PartialEq<&str> for FinancialProductIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FinancialProductIri || *other == FINANCIAL_PRODUCT_LABEL
	}
}
impl PartialEq<FinancialProductIriOrLabel> for &str {
	fn eq(&self, other: &FinancialProductIriOrLabel) -> bool {
		*self == FinancialProductIri || *self == FINANCIAL_PRODUCT_LABEL
	}
}
