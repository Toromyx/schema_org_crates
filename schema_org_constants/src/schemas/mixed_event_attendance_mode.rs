/// <https://schema.org/MixedEventAttendanceMode>
pub const MIXED_EVENT_ATTENDANCE_MODE_IRI_HTTP: &str = "http://schema.org/MixedEventAttendanceMode";
/// <https://schema.org/MixedEventAttendanceMode>
pub const MIXED_EVENT_ATTENDANCE_MODE_IRI_HTTPS: &str =
	"https://schema.org/MixedEventAttendanceMode";
/// <https://schema.org/MixedEventAttendanceMode>
pub const MIXED_EVENT_ATTENDANCE_MODE_LABEL: &str = "MixedEventAttendanceMode";
pub struct MixedEventAttendanceModeIri;
impl PartialEq<&str> for MixedEventAttendanceModeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MIXED_EVENT_ATTENDANCE_MODE_IRI_HTTP
			|| *other == MIXED_EVENT_ATTENDANCE_MODE_IRI_HTTPS
	}
}
impl PartialEq<MixedEventAttendanceModeIri> for &str {
	fn eq(&self, other: &MixedEventAttendanceModeIri) -> bool {
		*self == MIXED_EVENT_ATTENDANCE_MODE_IRI_HTTP
			|| *self == MIXED_EVENT_ATTENDANCE_MODE_IRI_HTTPS
	}
}
pub struct MixedEventAttendanceModeIriOrLabel;
impl PartialEq<&str> for MixedEventAttendanceModeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MixedEventAttendanceModeIri || *other == MIXED_EVENT_ATTENDANCE_MODE_LABEL
	}
}
impl PartialEq<MixedEventAttendanceModeIriOrLabel> for &str {
	fn eq(&self, other: &MixedEventAttendanceModeIriOrLabel) -> bool {
		*self == MixedEventAttendanceModeIri || *self == MIXED_EVENT_ATTENDANCE_MODE_LABEL
	}
}
