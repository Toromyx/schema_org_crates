/// <https://schema.org/salaryUponCompletion>
pub const SALARY_UPON_COMPLETION_PROPERTY_IRI_HTTP: &str = "http://schema.org/salaryUponCompletion";
/// <https://schema.org/salaryUponCompletion>
pub const SALARY_UPON_COMPLETION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/salaryUponCompletion";
/// <https://schema.org/salaryUponCompletion>
pub const SALARY_UPON_COMPLETION_PROPERTY_LABEL: &str = "salaryUponCompletion";
pub struct SalaryUponCompletionPropertyIri;
impl PartialEq<&str> for SalaryUponCompletionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SALARY_UPON_COMPLETION_PROPERTY_IRI_HTTP
			|| *other == SALARY_UPON_COMPLETION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SalaryUponCompletionPropertyIri> for &str {
	fn eq(&self, other: &SalaryUponCompletionPropertyIri) -> bool {
		*self == SALARY_UPON_COMPLETION_PROPERTY_IRI_HTTP
			|| *self == SALARY_UPON_COMPLETION_PROPERTY_IRI_HTTPS
	}
}
pub struct SalaryUponCompletionPropertyIriOrLabel;
impl PartialEq<&str> for SalaryUponCompletionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SalaryUponCompletionPropertyIri || *other == SALARY_UPON_COMPLETION_PROPERTY_LABEL
	}
}
impl PartialEq<SalaryUponCompletionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SalaryUponCompletionPropertyIriOrLabel) -> bool {
		*self == SalaryUponCompletionPropertyIri || *self == SALARY_UPON_COMPLETION_PROPERTY_LABEL
	}
}
