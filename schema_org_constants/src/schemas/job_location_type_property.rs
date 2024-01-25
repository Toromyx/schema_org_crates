/// <https://schema.org/jobLocationType>
pub const JOB_LOCATION_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/jobLocationType";
/// <https://schema.org/jobLocationType>
pub const JOB_LOCATION_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/jobLocationType";
/// <https://schema.org/jobLocationType>
pub const JOB_LOCATION_TYPE_PROPERTY_LABEL: &str = "jobLocationType";
pub struct JobLocationTypePropertyIri;
impl PartialEq<&str> for JobLocationTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOB_LOCATION_TYPE_PROPERTY_IRI_HTTP
			|| *other == JOB_LOCATION_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<JobLocationTypePropertyIri> for &str {
	fn eq(&self, other: &JobLocationTypePropertyIri) -> bool {
		*self == JOB_LOCATION_TYPE_PROPERTY_IRI_HTTP
			|| *self == JOB_LOCATION_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct JobLocationTypePropertyIriOrLabel;
impl PartialEq<&str> for JobLocationTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JobLocationTypePropertyIri || *other == JOB_LOCATION_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<JobLocationTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &JobLocationTypePropertyIriOrLabel) -> bool {
		*self == JobLocationTypePropertyIri || *self == JOB_LOCATION_TYPE_PROPERTY_LABEL
	}
}
