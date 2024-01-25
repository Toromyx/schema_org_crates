/// <https://schema.org/jobLocation>
pub const JOB_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/jobLocation";
/// <https://schema.org/jobLocation>
pub const JOB_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/jobLocation";
/// <https://schema.org/jobLocation>
pub const JOB_LOCATION_PROPERTY_LABEL: &str = "jobLocation";
pub struct JobLocationPropertyIri;
impl PartialEq<&str> for JobLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOB_LOCATION_PROPERTY_IRI_HTTP || *other == JOB_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<JobLocationPropertyIri> for &str {
	fn eq(&self, other: &JobLocationPropertyIri) -> bool {
		*self == JOB_LOCATION_PROPERTY_IRI_HTTP || *self == JOB_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct JobLocationPropertyIriOrLabel;
impl PartialEq<&str> for JobLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JobLocationPropertyIri || *other == JOB_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<JobLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &JobLocationPropertyIriOrLabel) -> bool {
		*self == JobLocationPropertyIri || *self == JOB_LOCATION_PROPERTY_LABEL
	}
}
