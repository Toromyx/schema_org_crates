/// <https://schema.org/baseSalary>
pub const BASE_SALARY_PROPERTY_IRI_HTTP: &str = "http://schema.org/baseSalary";
/// <https://schema.org/baseSalary>
pub const BASE_SALARY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/baseSalary";
/// <https://schema.org/baseSalary>
pub const BASE_SALARY_PROPERTY_LABEL: &str = "baseSalary";
pub struct BaseSalaryPropertyIri;
impl PartialEq<&str> for BaseSalaryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BASE_SALARY_PROPERTY_IRI_HTTP || *other == BASE_SALARY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BaseSalaryPropertyIri> for &str {
	fn eq(&self, other: &BaseSalaryPropertyIri) -> bool {
		*self == BASE_SALARY_PROPERTY_IRI_HTTP || *self == BASE_SALARY_PROPERTY_IRI_HTTPS
	}
}
pub struct BaseSalaryPropertyIriOrLabel;
impl PartialEq<&str> for BaseSalaryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BaseSalaryPropertyIri || *other == BASE_SALARY_PROPERTY_LABEL
	}
}
impl PartialEq<BaseSalaryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BaseSalaryPropertyIriOrLabel) -> bool {
		*self == BaseSalaryPropertyIri || *self == BASE_SALARY_PROPERTY_LABEL
	}
}
