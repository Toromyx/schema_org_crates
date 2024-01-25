/// <https://schema.org/financialAidEligible>
pub const FINANCIAL_AID_ELIGIBLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/financialAidEligible";
/// <https://schema.org/financialAidEligible>
pub const FINANCIAL_AID_ELIGIBLE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/financialAidEligible";
/// <https://schema.org/financialAidEligible>
pub const FINANCIAL_AID_ELIGIBLE_PROPERTY_LABEL: &str = "financialAidEligible";
pub struct FinancialAidEligiblePropertyIri;
impl PartialEq<&str> for FinancialAidEligiblePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FINANCIAL_AID_ELIGIBLE_PROPERTY_IRI_HTTP
			|| *other == FINANCIAL_AID_ELIGIBLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FinancialAidEligiblePropertyIri> for &str {
	fn eq(&self, other: &FinancialAidEligiblePropertyIri) -> bool {
		*self == FINANCIAL_AID_ELIGIBLE_PROPERTY_IRI_HTTP
			|| *self == FINANCIAL_AID_ELIGIBLE_PROPERTY_IRI_HTTPS
	}
}
pub struct FinancialAidEligiblePropertyIriOrLabel;
impl PartialEq<&str> for FinancialAidEligiblePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FinancialAidEligiblePropertyIri || *other == FINANCIAL_AID_ELIGIBLE_PROPERTY_LABEL
	}
}
impl PartialEq<FinancialAidEligiblePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FinancialAidEligiblePropertyIriOrLabel) -> bool {
		*self == FinancialAidEligiblePropertyIri || *self == FINANCIAL_AID_ELIGIBLE_PROPERTY_LABEL
	}
}
