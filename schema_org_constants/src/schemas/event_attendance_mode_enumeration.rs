/// <https://schema.org/EventAttendanceModeEnumeration>
pub const EVENT_ATTENDANCE_MODE_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/EventAttendanceModeEnumeration";
/// <https://schema.org/EventAttendanceModeEnumeration>
pub const EVENT_ATTENDANCE_MODE_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/EventAttendanceModeEnumeration";
/// <https://schema.org/EventAttendanceModeEnumeration>
pub const EVENT_ATTENDANCE_MODE_ENUMERATION_LABEL: &str = "EventAttendanceModeEnumeration";
pub struct EventAttendanceModeEnumerationIri;
impl PartialEq<&str> for EventAttendanceModeEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_ATTENDANCE_MODE_ENUMERATION_IRI_HTTP
			|| *other == EVENT_ATTENDANCE_MODE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<EventAttendanceModeEnumerationIri> for &str {
	fn eq(&self, other: &EventAttendanceModeEnumerationIri) -> bool {
		*self == EVENT_ATTENDANCE_MODE_ENUMERATION_IRI_HTTP
			|| *self == EVENT_ATTENDANCE_MODE_ENUMERATION_IRI_HTTPS
	}
}
pub struct EventAttendanceModeEnumerationIriOrLabel;
impl PartialEq<&str> for EventAttendanceModeEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventAttendanceModeEnumerationIri
			|| *other == EVENT_ATTENDANCE_MODE_ENUMERATION_LABEL
	}
}
impl PartialEq<EventAttendanceModeEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &EventAttendanceModeEnumerationIriOrLabel) -> bool {
		*self == EventAttendanceModeEnumerationIri
			|| *self == EVENT_ATTENDANCE_MODE_ENUMERATION_LABEL
	}
}
