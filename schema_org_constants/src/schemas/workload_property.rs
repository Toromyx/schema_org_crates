/// <https://schema.org/workload>
pub const WORKLOAD_PROPERTY_IRI_HTTP: &str = "http://schema.org/workload";
/// <https://schema.org/workload>
pub const WORKLOAD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/workload";
/// <https://schema.org/workload>
pub const WORKLOAD_PROPERTY_LABEL: &str = "workload";
pub struct WorkloadPropertyIri;
impl PartialEq<&str> for WorkloadPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORKLOAD_PROPERTY_IRI_HTTP || *other == WORKLOAD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorkloadPropertyIri> for &str {
	fn eq(&self, other: &WorkloadPropertyIri) -> bool {
		*self == WORKLOAD_PROPERTY_IRI_HTTP || *self == WORKLOAD_PROPERTY_IRI_HTTPS
	}
}
pub struct WorkloadPropertyIriOrLabel;
impl PartialEq<&str> for WorkloadPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkloadPropertyIri || *other == WORKLOAD_PROPERTY_LABEL
	}
}
impl PartialEq<WorkloadPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorkloadPropertyIriOrLabel) -> bool {
		*self == WorkloadPropertyIri || *self == WORKLOAD_PROPERTY_LABEL
	}
}
