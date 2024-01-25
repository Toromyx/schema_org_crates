/// <https://schema.org/workPerformed>
pub const WORK_PERFORMED_PROPERTY_IRI_HTTP: &str = "http://schema.org/workPerformed";
/// <https://schema.org/workPerformed>
pub const WORK_PERFORMED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/workPerformed";
/// <https://schema.org/workPerformed>
pub const WORK_PERFORMED_PROPERTY_LABEL: &str = "workPerformed";
pub struct WorkPerformedPropertyIri;
impl PartialEq<&str> for WorkPerformedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORK_PERFORMED_PROPERTY_IRI_HTTP || *other == WORK_PERFORMED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorkPerformedPropertyIri> for &str {
	fn eq(&self, other: &WorkPerformedPropertyIri) -> bool {
		*self == WORK_PERFORMED_PROPERTY_IRI_HTTP || *self == WORK_PERFORMED_PROPERTY_IRI_HTTPS
	}
}
pub struct WorkPerformedPropertyIriOrLabel;
impl PartialEq<&str> for WorkPerformedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkPerformedPropertyIri || *other == WORK_PERFORMED_PROPERTY_LABEL
	}
}
impl PartialEq<WorkPerformedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorkPerformedPropertyIriOrLabel) -> bool {
		*self == WorkPerformedPropertyIri || *self == WORK_PERFORMED_PROPERTY_LABEL
	}
}
