/// <https://schema.org/salaryCurrency>
pub const SALARY_CURRENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/salaryCurrency";
/// <https://schema.org/salaryCurrency>
pub const SALARY_CURRENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/salaryCurrency";
/// <https://schema.org/salaryCurrency>
pub const SALARY_CURRENCY_PROPERTY_LABEL: &str = "salaryCurrency";
pub struct SalaryCurrencyPropertyIri;
impl PartialEq<&str> for SalaryCurrencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SALARY_CURRENCY_PROPERTY_IRI_HTTP || *other == SALARY_CURRENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SalaryCurrencyPropertyIri> for &str {
	fn eq(&self, other: &SalaryCurrencyPropertyIri) -> bool {
		*self == SALARY_CURRENCY_PROPERTY_IRI_HTTP || *self == SALARY_CURRENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct SalaryCurrencyPropertyIriOrLabel;
impl PartialEq<&str> for SalaryCurrencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SalaryCurrencyPropertyIri || *other == SALARY_CURRENCY_PROPERTY_LABEL
	}
}
impl PartialEq<SalaryCurrencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SalaryCurrencyPropertyIriOrLabel) -> bool {
		*self == SalaryCurrencyPropertyIri || *self == SALARY_CURRENCY_PROPERTY_LABEL
	}
}
