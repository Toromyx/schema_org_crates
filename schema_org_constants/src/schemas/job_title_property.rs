/// <https://schema.org/jobTitle>
pub const JOB_TITLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/jobTitle";
/// <https://schema.org/jobTitle>
pub const JOB_TITLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/jobTitle";
/// <https://schema.org/jobTitle>
pub const JOB_TITLE_PROPERTY_LABEL: &str = "jobTitle";
pub struct JobTitlePropertyIri;
impl PartialEq<&str> for JobTitlePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOB_TITLE_PROPERTY_IRI_HTTP || *other == JOB_TITLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<JobTitlePropertyIri> for &str {
	fn eq(&self, other: &JobTitlePropertyIri) -> bool {
		*self == JOB_TITLE_PROPERTY_IRI_HTTP || *self == JOB_TITLE_PROPERTY_IRI_HTTPS
	}
}
pub struct JobTitlePropertyIriOrLabel;
impl PartialEq<&str> for JobTitlePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JobTitlePropertyIri || *other == JOB_TITLE_PROPERTY_LABEL
	}
}
impl PartialEq<JobTitlePropertyIriOrLabel> for &str {
	fn eq(&self, other: &JobTitlePropertyIriOrLabel) -> bool {
		*self == JobTitlePropertyIri || *self == JOB_TITLE_PROPERTY_LABEL
	}
}
