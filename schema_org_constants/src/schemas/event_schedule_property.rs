/// <https://schema.org/eventSchedule>
pub const EVENT_SCHEDULE_PROPERTY_IRI_HTTP: &str = "http://schema.org/eventSchedule";
/// <https://schema.org/eventSchedule>
pub const EVENT_SCHEDULE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/eventSchedule";
/// <https://schema.org/eventSchedule>
pub const EVENT_SCHEDULE_PROPERTY_LABEL: &str = "eventSchedule";
pub struct EventSchedulePropertyIri;
impl PartialEq<&str> for EventSchedulePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_SCHEDULE_PROPERTY_IRI_HTTP || *other == EVENT_SCHEDULE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EventSchedulePropertyIri> for &str {
	fn eq(&self, other: &EventSchedulePropertyIri) -> bool {
		*self == EVENT_SCHEDULE_PROPERTY_IRI_HTTP || *self == EVENT_SCHEDULE_PROPERTY_IRI_HTTPS
	}
}
pub struct EventSchedulePropertyIriOrLabel;
impl PartialEq<&str> for EventSchedulePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventSchedulePropertyIri || *other == EVENT_SCHEDULE_PROPERTY_LABEL
	}
}
impl PartialEq<EventSchedulePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EventSchedulePropertyIriOrLabel) -> bool {
		*self == EventSchedulePropertyIri || *self == EVENT_SCHEDULE_PROPERTY_LABEL
	}
}
