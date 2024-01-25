/// <https://schema.org/ScheduleAction>
pub const SCHEDULE_ACTION_IRI_HTTP: &str = "http://schema.org/ScheduleAction";
/// <https://schema.org/ScheduleAction>
pub const SCHEDULE_ACTION_IRI_HTTPS: &str = "https://schema.org/ScheduleAction";
/// <https://schema.org/ScheduleAction>
pub const SCHEDULE_ACTION_LABEL: &str = "ScheduleAction";
pub struct ScheduleActionIri;
impl PartialEq<&str> for ScheduleActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHEDULE_ACTION_IRI_HTTP || *other == SCHEDULE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ScheduleActionIri> for &str {
	fn eq(&self, other: &ScheduleActionIri) -> bool {
		*self == SCHEDULE_ACTION_IRI_HTTP || *self == SCHEDULE_ACTION_IRI_HTTPS
	}
}
pub struct ScheduleActionIriOrLabel;
impl PartialEq<&str> for ScheduleActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScheduleActionIri || *other == SCHEDULE_ACTION_LABEL
	}
}
impl PartialEq<ScheduleActionIriOrLabel> for &str {
	fn eq(&self, other: &ScheduleActionIriOrLabel) -> bool {
		*self == ScheduleActionIri || *self == SCHEDULE_ACTION_LABEL
	}
}
