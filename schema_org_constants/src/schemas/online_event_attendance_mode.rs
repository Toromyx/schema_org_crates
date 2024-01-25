/// <https://schema.org/OnlineEventAttendanceMode>
pub const ONLINE_EVENT_ATTENDANCE_MODE_IRI_HTTP: &str =
	"http://schema.org/OnlineEventAttendanceMode";
/// <https://schema.org/OnlineEventAttendanceMode>
pub const ONLINE_EVENT_ATTENDANCE_MODE_IRI_HTTPS: &str =
	"https://schema.org/OnlineEventAttendanceMode";
/// <https://schema.org/OnlineEventAttendanceMode>
pub const ONLINE_EVENT_ATTENDANCE_MODE_LABEL: &str = "OnlineEventAttendanceMode";
pub struct OnlineEventAttendanceModeIri;
impl PartialEq<&str> for OnlineEventAttendanceModeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ONLINE_EVENT_ATTENDANCE_MODE_IRI_HTTP
			|| *other == ONLINE_EVENT_ATTENDANCE_MODE_IRI_HTTPS
	}
}
impl PartialEq<OnlineEventAttendanceModeIri> for &str {
	fn eq(&self, other: &OnlineEventAttendanceModeIri) -> bool {
		*self == ONLINE_EVENT_ATTENDANCE_MODE_IRI_HTTP
			|| *self == ONLINE_EVENT_ATTENDANCE_MODE_IRI_HTTPS
	}
}
pub struct OnlineEventAttendanceModeIriOrLabel;
impl PartialEq<&str> for OnlineEventAttendanceModeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OnlineEventAttendanceModeIri || *other == ONLINE_EVENT_ATTENDANCE_MODE_LABEL
	}
}
impl PartialEq<OnlineEventAttendanceModeIriOrLabel> for &str {
	fn eq(&self, other: &OnlineEventAttendanceModeIriOrLabel) -> bool {
		*self == OnlineEventAttendanceModeIri || *self == ONLINE_EVENT_ATTENDANCE_MODE_LABEL
	}
}
