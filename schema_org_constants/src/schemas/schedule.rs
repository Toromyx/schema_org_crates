/// <https://schema.org/Schedule>
pub const SCHEDULE_IRI_HTTP: &str = "http://schema.org/Schedule";
/// <https://schema.org/Schedule>
pub const SCHEDULE_IRI_HTTPS: &str = "https://schema.org/Schedule";
/// <https://schema.org/Schedule>
pub const SCHEDULE_LABEL: &str = "Schedule";
pub struct ScheduleIri;
impl PartialEq<&str> for ScheduleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHEDULE_IRI_HTTP || *other == SCHEDULE_IRI_HTTPS
	}
}
impl PartialEq<ScheduleIri> for &str {
	fn eq(&self, other: &ScheduleIri) -> bool {
		*self == SCHEDULE_IRI_HTTP || *self == SCHEDULE_IRI_HTTPS
	}
}
pub struct ScheduleIriOrLabel;
impl PartialEq<&str> for ScheduleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScheduleIri || *other == SCHEDULE_LABEL
	}
}
impl PartialEq<ScheduleIriOrLabel> for &str {
	fn eq(&self, other: &ScheduleIriOrLabel) -> bool {
		*self == ScheduleIri || *self == SCHEDULE_LABEL
	}
}
