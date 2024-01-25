/// <https://schema.org/FinancialService>
pub const FINANCIAL_SERVICE_IRI_HTTP: &str = "http://schema.org/FinancialService";
/// <https://schema.org/FinancialService>
pub const FINANCIAL_SERVICE_IRI_HTTPS: &str = "https://schema.org/FinancialService";
/// <https://schema.org/FinancialService>
pub const FINANCIAL_SERVICE_LABEL: &str = "FinancialService";
pub struct FinancialServiceIri;
impl PartialEq<&str> for FinancialServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FINANCIAL_SERVICE_IRI_HTTP || *other == FINANCIAL_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<FinancialServiceIri> for &str {
	fn eq(&self, other: &FinancialServiceIri) -> bool {
		*self == FINANCIAL_SERVICE_IRI_HTTP || *self == FINANCIAL_SERVICE_IRI_HTTPS
	}
}
pub struct FinancialServiceIriOrLabel;
impl PartialEq<&str> for FinancialServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FinancialServiceIri || *other == FINANCIAL_SERVICE_LABEL
	}
}
impl PartialEq<FinancialServiceIriOrLabel> for &str {
	fn eq(&self, other: &FinancialServiceIriOrLabel) -> bool {
		*self == FinancialServiceIri || *self == FINANCIAL_SERVICE_LABEL
	}
}
