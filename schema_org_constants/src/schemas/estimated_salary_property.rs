/// <https://schema.org/estimatedSalary>
pub const ESTIMATED_SALARY_PROPERTY_IRI_HTTP: &str = "http://schema.org/estimatedSalary";
/// <https://schema.org/estimatedSalary>
pub const ESTIMATED_SALARY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/estimatedSalary";
/// <https://schema.org/estimatedSalary>
pub const ESTIMATED_SALARY_PROPERTY_LABEL: &str = "estimatedSalary";
pub struct EstimatedSalaryPropertyIri;
impl PartialEq<&str> for EstimatedSalaryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ESTIMATED_SALARY_PROPERTY_IRI_HTTP
			|| *other == ESTIMATED_SALARY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EstimatedSalaryPropertyIri> for &str {
	fn eq(&self, other: &EstimatedSalaryPropertyIri) -> bool {
		*self == ESTIMATED_SALARY_PROPERTY_IRI_HTTP || *self == ESTIMATED_SALARY_PROPERTY_IRI_HTTPS
	}
}
pub struct EstimatedSalaryPropertyIriOrLabel;
impl PartialEq<&str> for EstimatedSalaryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EstimatedSalaryPropertyIri || *other == ESTIMATED_SALARY_PROPERTY_LABEL
	}
}
impl PartialEq<EstimatedSalaryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EstimatedSalaryPropertyIriOrLabel) -> bool {
		*self == EstimatedSalaryPropertyIri || *self == ESTIMATED_SALARY_PROPERTY_LABEL
	}
}
