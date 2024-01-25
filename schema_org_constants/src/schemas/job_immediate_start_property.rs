/// <https://schema.org/jobImmediateStart>
pub const JOB_IMMEDIATE_START_PROPERTY_IRI_HTTP: &str = "http://schema.org/jobImmediateStart";
/// <https://schema.org/jobImmediateStart>
pub const JOB_IMMEDIATE_START_PROPERTY_IRI_HTTPS: &str = "https://schema.org/jobImmediateStart";
/// <https://schema.org/jobImmediateStart>
pub const JOB_IMMEDIATE_START_PROPERTY_LABEL: &str = "jobImmediateStart";
pub struct JobImmediateStartPropertyIri;
impl PartialEq<&str> for JobImmediateStartPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOB_IMMEDIATE_START_PROPERTY_IRI_HTTP
			|| *other == JOB_IMMEDIATE_START_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<JobImmediateStartPropertyIri> for &str {
	fn eq(&self, other: &JobImmediateStartPropertyIri) -> bool {
		*self == JOB_IMMEDIATE_START_PROPERTY_IRI_HTTP
			|| *self == JOB_IMMEDIATE_START_PROPERTY_IRI_HTTPS
	}
}
pub struct JobImmediateStartPropertyIriOrLabel;
impl PartialEq<&str> for JobImmediateStartPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JobImmediateStartPropertyIri || *other == JOB_IMMEDIATE_START_PROPERTY_LABEL
	}
}
impl PartialEq<JobImmediateStartPropertyIriOrLabel> for &str {
	fn eq(&self, other: &JobImmediateStartPropertyIriOrLabel) -> bool {
		*self == JobImmediateStartPropertyIri || *self == JOB_IMMEDIATE_START_PROPERTY_LABEL
	}
}
