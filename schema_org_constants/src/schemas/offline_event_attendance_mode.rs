/// <https://schema.org/OfflineEventAttendanceMode>
pub const OFFLINE_EVENT_ATTENDANCE_MODE_IRI_HTTP: &str =
	"http://schema.org/OfflineEventAttendanceMode";
/// <https://schema.org/OfflineEventAttendanceMode>
pub const OFFLINE_EVENT_ATTENDANCE_MODE_IRI_HTTPS: &str =
	"https://schema.org/OfflineEventAttendanceMode";
/// <https://schema.org/OfflineEventAttendanceMode>
pub const OFFLINE_EVENT_ATTENDANCE_MODE_LABEL: &str = "OfflineEventAttendanceMode";
pub struct OfflineEventAttendanceModeIri;
impl PartialEq<&str> for OfflineEventAttendanceModeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFLINE_EVENT_ATTENDANCE_MODE_IRI_HTTP
			|| *other == OFFLINE_EVENT_ATTENDANCE_MODE_IRI_HTTPS
	}
}
impl PartialEq<OfflineEventAttendanceModeIri> for &str {
	fn eq(&self, other: &OfflineEventAttendanceModeIri) -> bool {
		*self == OFFLINE_EVENT_ATTENDANCE_MODE_IRI_HTTP
			|| *self == OFFLINE_EVENT_ATTENDANCE_MODE_IRI_HTTPS
	}
}
pub struct OfflineEventAttendanceModeIriOrLabel;
impl PartialEq<&str> for OfflineEventAttendanceModeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfflineEventAttendanceModeIri || *other == OFFLINE_EVENT_ATTENDANCE_MODE_LABEL
	}
}
impl PartialEq<OfflineEventAttendanceModeIriOrLabel> for &str {
	fn eq(&self, other: &OfflineEventAttendanceModeIriOrLabel) -> bool {
		*self == OfflineEventAttendanceModeIri || *self == OFFLINE_EVENT_ATTENDANCE_MODE_LABEL
	}
}
