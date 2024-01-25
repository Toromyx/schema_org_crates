/// <https://schema.org/eventAttendanceMode>
pub const EVENT_ATTENDANCE_MODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/eventAttendanceMode";
/// <https://schema.org/eventAttendanceMode>
pub const EVENT_ATTENDANCE_MODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/eventAttendanceMode";
/// <https://schema.org/eventAttendanceMode>
pub const EVENT_ATTENDANCE_MODE_PROPERTY_LABEL: &str = "eventAttendanceMode";
pub struct EventAttendanceModePropertyIri;
impl PartialEq<&str> for EventAttendanceModePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_ATTENDANCE_MODE_PROPERTY_IRI_HTTP
			|| *other == EVENT_ATTENDANCE_MODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EventAttendanceModePropertyIri> for &str {
	fn eq(&self, other: &EventAttendanceModePropertyIri) -> bool {
		*self == EVENT_ATTENDANCE_MODE_PROPERTY_IRI_HTTP
			|| *self == EVENT_ATTENDANCE_MODE_PROPERTY_IRI_HTTPS
	}
}
pub struct EventAttendanceModePropertyIriOrLabel;
impl PartialEq<&str> for EventAttendanceModePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventAttendanceModePropertyIri || *other == EVENT_ATTENDANCE_MODE_PROPERTY_LABEL
	}
}
impl PartialEq<EventAttendanceModePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EventAttendanceModePropertyIriOrLabel) -> bool {
		*self == EventAttendanceModePropertyIri || *self == EVENT_ATTENDANCE_MODE_PROPERTY_LABEL
	}
}
