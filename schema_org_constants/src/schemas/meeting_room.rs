/// <https://schema.org/MeetingRoom>
pub const MEETING_ROOM_IRI_HTTP: &str = "http://schema.org/MeetingRoom";
/// <https://schema.org/MeetingRoom>
pub const MEETING_ROOM_IRI_HTTPS: &str = "https://schema.org/MeetingRoom";
/// <https://schema.org/MeetingRoom>
pub const MEETING_ROOM_LABEL: &str = "MeetingRoom";
pub struct MeetingRoomIri;
impl PartialEq<&str> for MeetingRoomIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEETING_ROOM_IRI_HTTP || *other == MEETING_ROOM_IRI_HTTPS
	}
}
impl PartialEq<MeetingRoomIri> for &str {
	fn eq(&self, other: &MeetingRoomIri) -> bool {
		*self == MEETING_ROOM_IRI_HTTP || *self == MEETING_ROOM_IRI_HTTPS
	}
}
pub struct MeetingRoomIriOrLabel;
impl PartialEq<&str> for MeetingRoomIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeetingRoomIri || *other == MEETING_ROOM_LABEL
	}
}
impl PartialEq<MeetingRoomIriOrLabel> for &str {
	fn eq(&self, other: &MeetingRoomIriOrLabel) -> bool {
		*self == MeetingRoomIri || *self == MEETING_ROOM_LABEL
	}
}
