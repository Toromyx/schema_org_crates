/// <https://schema.org/scheduleTimezone>
pub const SCHEDULE_TIMEZONE_PROPERTY_IRI_HTTP: &str = "http://schema.org/scheduleTimezone";
/// <https://schema.org/scheduleTimezone>
pub const SCHEDULE_TIMEZONE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/scheduleTimezone";
/// <https://schema.org/scheduleTimezone>
pub const SCHEDULE_TIMEZONE_PROPERTY_LABEL: &str = "scheduleTimezone";
pub struct ScheduleTimezonePropertyIri;
impl PartialEq<&str> for ScheduleTimezonePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHEDULE_TIMEZONE_PROPERTY_IRI_HTTP
			|| *other == SCHEDULE_TIMEZONE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ScheduleTimezonePropertyIri> for &str {
	fn eq(&self, other: &ScheduleTimezonePropertyIri) -> bool {
		*self == SCHEDULE_TIMEZONE_PROPERTY_IRI_HTTP
			|| *self == SCHEDULE_TIMEZONE_PROPERTY_IRI_HTTPS
	}
}
pub struct ScheduleTimezonePropertyIriOrLabel;
impl PartialEq<&str> for ScheduleTimezonePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScheduleTimezonePropertyIri || *other == SCHEDULE_TIMEZONE_PROPERTY_LABEL
	}
}
impl PartialEq<ScheduleTimezonePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ScheduleTimezonePropertyIriOrLabel) -> bool {
		*self == ScheduleTimezonePropertyIri || *self == SCHEDULE_TIMEZONE_PROPERTY_LABEL
	}
}
