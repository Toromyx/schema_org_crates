/// <https://schema.org/workLocation>
pub const WORK_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/workLocation";
/// <https://schema.org/workLocation>
pub const WORK_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/workLocation";
/// <https://schema.org/workLocation>
pub const WORK_LOCATION_PROPERTY_LABEL: &str = "workLocation";
pub struct WorkLocationPropertyIri;
impl PartialEq<&str> for WorkLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORK_LOCATION_PROPERTY_IRI_HTTP || *other == WORK_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorkLocationPropertyIri> for &str {
	fn eq(&self, other: &WorkLocationPropertyIri) -> bool {
		*self == WORK_LOCATION_PROPERTY_IRI_HTTP || *self == WORK_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct WorkLocationPropertyIriOrLabel;
impl PartialEq<&str> for WorkLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkLocationPropertyIri || *other == WORK_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<WorkLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorkLocationPropertyIriOrLabel) -> bool {
		*self == WorkLocationPropertyIri || *self == WORK_LOCATION_PROPERTY_LABEL
	}
}
