/// <https://schema.org/Room>
pub const ROOM_IRI_HTTP: &str = "http://schema.org/Room";
/// <https://schema.org/Room>
pub const ROOM_IRI_HTTPS: &str = "https://schema.org/Room";
/// <https://schema.org/Room>
pub const ROOM_LABEL: &str = "Room";
pub struct RoomIri;
impl PartialEq<&str> for RoomIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ROOM_IRI_HTTP || *other == ROOM_IRI_HTTPS
	}
}
impl PartialEq<RoomIri> for &str {
	fn eq(&self, other: &RoomIri) -> bool {
		*self == ROOM_IRI_HTTP || *self == ROOM_IRI_HTTPS
	}
}
pub struct RoomIriOrLabel;
impl PartialEq<&str> for RoomIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RoomIri || *other == ROOM_LABEL
	}
}
impl PartialEq<RoomIriOrLabel> for &str {
	fn eq(&self, other: &RoomIriOrLabel) -> bool {
		*self == RoomIri || *self == ROOM_LABEL
	}
}
