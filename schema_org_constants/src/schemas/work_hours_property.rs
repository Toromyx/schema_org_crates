/// <https://schema.org/workHours>
pub const WORK_HOURS_PROPERTY_IRI_HTTP: &str = "http://schema.org/workHours";
/// <https://schema.org/workHours>
pub const WORK_HOURS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/workHours";
/// <https://schema.org/workHours>
pub const WORK_HOURS_PROPERTY_LABEL: &str = "workHours";
pub struct WorkHoursPropertyIri;
impl PartialEq<&str> for WorkHoursPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORK_HOURS_PROPERTY_IRI_HTTP || *other == WORK_HOURS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorkHoursPropertyIri> for &str {
	fn eq(&self, other: &WorkHoursPropertyIri) -> bool {
		*self == WORK_HOURS_PROPERTY_IRI_HTTP || *self == WORK_HOURS_PROPERTY_IRI_HTTPS
	}
}
pub struct WorkHoursPropertyIriOrLabel;
impl PartialEq<&str> for WorkHoursPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkHoursPropertyIri || *other == WORK_HOURS_PROPERTY_LABEL
	}
}
impl PartialEq<WorkHoursPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorkHoursPropertyIriOrLabel) -> bool {
		*self == WorkHoursPropertyIri || *self == WORK_HOURS_PROPERTY_LABEL
	}
}
