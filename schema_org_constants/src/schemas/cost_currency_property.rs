/// <https://schema.org/costCurrency>
pub const COST_CURRENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/costCurrency";
/// <https://schema.org/costCurrency>
pub const COST_CURRENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/costCurrency";
/// <https://schema.org/costCurrency>
pub const COST_CURRENCY_PROPERTY_LABEL: &str = "costCurrency";
pub struct CostCurrencyPropertyIri;
impl PartialEq<&str> for CostCurrencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COST_CURRENCY_PROPERTY_IRI_HTTP || *other == COST_CURRENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CostCurrencyPropertyIri> for &str {
	fn eq(&self, other: &CostCurrencyPropertyIri) -> bool {
		*self == COST_CURRENCY_PROPERTY_IRI_HTTP || *self == COST_CURRENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct CostCurrencyPropertyIriOrLabel;
impl PartialEq<&str> for CostCurrencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CostCurrencyPropertyIri || *other == COST_CURRENCY_PROPERTY_LABEL
	}
}
impl PartialEq<CostCurrencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CostCurrencyPropertyIriOrLabel) -> bool {
		*self == CostCurrencyPropertyIri || *self == COST_CURRENCY_PROPERTY_LABEL
	}
}
