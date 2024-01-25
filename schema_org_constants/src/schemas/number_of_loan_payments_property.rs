/// <https://schema.org/numberOfLoanPayments>
pub const NUMBER_OF_LOAN_PAYMENTS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/numberOfLoanPayments";
/// <https://schema.org/numberOfLoanPayments>
pub const NUMBER_OF_LOAN_PAYMENTS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/numberOfLoanPayments";
/// <https://schema.org/numberOfLoanPayments>
pub const NUMBER_OF_LOAN_PAYMENTS_PROPERTY_LABEL: &str = "numberOfLoanPayments";
pub struct NumberOfLoanPaymentsPropertyIri;
impl PartialEq<&str> for NumberOfLoanPaymentsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_LOAN_PAYMENTS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_LOAN_PAYMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfLoanPaymentsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfLoanPaymentsPropertyIri) -> bool {
		*self == NUMBER_OF_LOAN_PAYMENTS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_LOAN_PAYMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfLoanPaymentsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfLoanPaymentsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfLoanPaymentsPropertyIri
			|| *other == NUMBER_OF_LOAN_PAYMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfLoanPaymentsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfLoanPaymentsPropertyIriOrLabel) -> bool {
		*self == NumberOfLoanPaymentsPropertyIri || *self == NUMBER_OF_LOAN_PAYMENTS_PROPERTY_LABEL
	}
}
