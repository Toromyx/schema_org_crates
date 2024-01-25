/// <https://schema.org/workExample>
pub const WORK_EXAMPLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/workExample";
/// <https://schema.org/workExample>
pub const WORK_EXAMPLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/workExample";
/// <https://schema.org/workExample>
pub const WORK_EXAMPLE_PROPERTY_LABEL: &str = "workExample";
pub struct WorkExamplePropertyIri;
impl PartialEq<&str> for WorkExamplePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORK_EXAMPLE_PROPERTY_IRI_HTTP || *other == WORK_EXAMPLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorkExamplePropertyIri> for &str {
	fn eq(&self, other: &WorkExamplePropertyIri) -> bool {
		*self == WORK_EXAMPLE_PROPERTY_IRI_HTTP || *self == WORK_EXAMPLE_PROPERTY_IRI_HTTPS
	}
}
pub struct WorkExamplePropertyIriOrLabel;
impl PartialEq<&str> for WorkExamplePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkExamplePropertyIri || *other == WORK_EXAMPLE_PROPERTY_LABEL
	}
}
impl PartialEq<WorkExamplePropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorkExamplePropertyIriOrLabel) -> bool {
		*self == WorkExamplePropertyIri || *self == WORK_EXAMPLE_PROPERTY_LABEL
	}
}
