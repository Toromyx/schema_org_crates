/// <https://schema.org/trainingSalary>
pub const TRAINING_SALARY_PROPERTY_IRI_HTTP: &str = "http://schema.org/trainingSalary";
/// <https://schema.org/trainingSalary>
pub const TRAINING_SALARY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/trainingSalary";
/// <https://schema.org/trainingSalary>
pub const TRAINING_SALARY_PROPERTY_LABEL: &str = "trainingSalary";
pub struct TrainingSalaryPropertyIri;
impl PartialEq<&str> for TrainingSalaryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAINING_SALARY_PROPERTY_IRI_HTTP || *other == TRAINING_SALARY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrainingSalaryPropertyIri> for &str {
	fn eq(&self, other: &TrainingSalaryPropertyIri) -> bool {
		*self == TRAINING_SALARY_PROPERTY_IRI_HTTP || *self == TRAINING_SALARY_PROPERTY_IRI_HTTPS
	}
}
pub struct TrainingSalaryPropertyIriOrLabel;
impl PartialEq<&str> for TrainingSalaryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrainingSalaryPropertyIri || *other == TRAINING_SALARY_PROPERTY_LABEL
	}
}
impl PartialEq<TrainingSalaryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrainingSalaryPropertyIriOrLabel) -> bool {
		*self == TrainingSalaryPropertyIri || *self == TRAINING_SALARY_PROPERTY_LABEL
	}
}
