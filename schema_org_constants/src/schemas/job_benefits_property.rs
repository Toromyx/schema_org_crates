/// <https://schema.org/jobBenefits>
pub const JOB_BENEFITS_PROPERTY_IRI_HTTP: &str = "http://schema.org/jobBenefits";
/// <https://schema.org/jobBenefits>
pub const JOB_BENEFITS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/jobBenefits";
/// <https://schema.org/jobBenefits>
pub const JOB_BENEFITS_PROPERTY_LABEL: &str = "jobBenefits";
pub struct JobBenefitsPropertyIri;
impl PartialEq<&str> for JobBenefitsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOB_BENEFITS_PROPERTY_IRI_HTTP || *other == JOB_BENEFITS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<JobBenefitsPropertyIri> for &str {
	fn eq(&self, other: &JobBenefitsPropertyIri) -> bool {
		*self == JOB_BENEFITS_PROPERTY_IRI_HTTP || *self == JOB_BENEFITS_PROPERTY_IRI_HTTPS
	}
}
pub struct JobBenefitsPropertyIriOrLabel;
impl PartialEq<&str> for JobBenefitsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JobBenefitsPropertyIri || *other == JOB_BENEFITS_PROPERTY_LABEL
	}
}
impl PartialEq<JobBenefitsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &JobBenefitsPropertyIriOrLabel) -> bool {
		*self == JobBenefitsPropertyIri || *self == JOB_BENEFITS_PROPERTY_LABEL
	}
}
