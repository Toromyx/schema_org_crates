/// <https://schema.org/jobStartDate>
pub const JOB_START_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/jobStartDate";
/// <https://schema.org/jobStartDate>
pub const JOB_START_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/jobStartDate";
/// <https://schema.org/jobStartDate>
pub const JOB_START_DATE_PROPERTY_LABEL: &str = "jobStartDate";
pub struct JobStartDatePropertyIri;
impl PartialEq<&str> for JobStartDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOB_START_DATE_PROPERTY_IRI_HTTP || *other == JOB_START_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<JobStartDatePropertyIri> for &str {
	fn eq(&self, other: &JobStartDatePropertyIri) -> bool {
		*self == JOB_START_DATE_PROPERTY_IRI_HTTP || *self == JOB_START_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct JobStartDatePropertyIriOrLabel;
impl PartialEq<&str> for JobStartDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JobStartDatePropertyIri || *other == JOB_START_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<JobStartDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &JobStartDatePropertyIriOrLabel) -> bool {
		*self == JobStartDatePropertyIri || *self == JOB_START_DATE_PROPERTY_LABEL
	}
}
