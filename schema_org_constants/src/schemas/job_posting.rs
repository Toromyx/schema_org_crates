/// <https://schema.org/JobPosting>
pub const JOB_POSTING_IRI_HTTP: &str = "http://schema.org/JobPosting";
/// <https://schema.org/JobPosting>
pub const JOB_POSTING_IRI_HTTPS: &str = "https://schema.org/JobPosting";
/// <https://schema.org/JobPosting>
pub const JOB_POSTING_LABEL: &str = "JobPosting";
pub struct JobPostingIri;
impl PartialEq<&str> for JobPostingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOB_POSTING_IRI_HTTP || *other == JOB_POSTING_IRI_HTTPS
	}
}
impl PartialEq<JobPostingIri> for &str {
	fn eq(&self, other: &JobPostingIri) -> bool {
		*self == JOB_POSTING_IRI_HTTP || *self == JOB_POSTING_IRI_HTTPS
	}
}
pub struct JobPostingIriOrLabel;
impl PartialEq<&str> for JobPostingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JobPostingIri || *other == JOB_POSTING_LABEL
	}
}
impl PartialEq<JobPostingIriOrLabel> for &str {
	fn eq(&self, other: &JobPostingIriOrLabel) -> bool {
		*self == JobPostingIri || *self == JOB_POSTING_LABEL
	}
}
