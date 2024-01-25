/// <https://schema.org/BasicIncome>
pub const BASIC_INCOME_IRI_HTTP: &str = "http://schema.org/BasicIncome";
/// <https://schema.org/BasicIncome>
pub const BASIC_INCOME_IRI_HTTPS: &str = "https://schema.org/BasicIncome";
/// <https://schema.org/BasicIncome>
pub const BASIC_INCOME_LABEL: &str = "BasicIncome";
pub struct BasicIncomeIri;
impl PartialEq<&str> for BasicIncomeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BASIC_INCOME_IRI_HTTP || *other == BASIC_INCOME_IRI_HTTPS
	}
}
impl PartialEq<BasicIncomeIri> for &str {
	fn eq(&self, other: &BasicIncomeIri) -> bool {
		*self == BASIC_INCOME_IRI_HTTP || *self == BASIC_INCOME_IRI_HTTPS
	}
}
pub struct BasicIncomeIriOrLabel;
impl PartialEq<&str> for BasicIncomeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BasicIncomeIri || *other == BASIC_INCOME_LABEL
	}
}
impl PartialEq<BasicIncomeIriOrLabel> for &str {
	fn eq(&self, other: &BasicIncomeIriOrLabel) -> bool {
		*self == BasicIncomeIri || *self == BASIC_INCOME_LABEL
	}
}
