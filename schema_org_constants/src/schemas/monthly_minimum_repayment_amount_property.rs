/// <https://schema.org/monthlyMinimumRepaymentAmount>
pub const MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/monthlyMinimumRepaymentAmount";
/// <https://schema.org/monthlyMinimumRepaymentAmount>
pub const MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/monthlyMinimumRepaymentAmount";
/// <https://schema.org/monthlyMinimumRepaymentAmount>
pub const MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_LABEL: &str = "monthlyMinimumRepaymentAmount";
pub struct MonthlyMinimumRepaymentAmountPropertyIri;
impl PartialEq<&str> for MonthlyMinimumRepaymentAmountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_IRI_HTTP
			|| *other == MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MonthlyMinimumRepaymentAmountPropertyIri> for &str {
	fn eq(&self, other: &MonthlyMinimumRepaymentAmountPropertyIri) -> bool {
		*self == MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_IRI_HTTP
			|| *self == MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct MonthlyMinimumRepaymentAmountPropertyIriOrLabel;
impl PartialEq<&str> for MonthlyMinimumRepaymentAmountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MonthlyMinimumRepaymentAmountPropertyIri
			|| *other == MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_LABEL
	}
}
impl PartialEq<MonthlyMinimumRepaymentAmountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MonthlyMinimumRepaymentAmountPropertyIriOrLabel) -> bool {
		*self == MonthlyMinimumRepaymentAmountPropertyIri
			|| *self == MONTHLY_MINIMUM_REPAYMENT_AMOUNT_PROPERTY_LABEL
	}
}
