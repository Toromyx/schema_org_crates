/// <https://schema.org/HotelRoom>
pub const HOTEL_ROOM_IRI_HTTP: &str = "http://schema.org/HotelRoom";
/// <https://schema.org/HotelRoom>
pub const HOTEL_ROOM_IRI_HTTPS: &str = "https://schema.org/HotelRoom";
/// <https://schema.org/HotelRoom>
pub const HOTEL_ROOM_LABEL: &str = "HotelRoom";
pub struct HotelRoomIri;
impl PartialEq<&str> for HotelRoomIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOTEL_ROOM_IRI_HTTP || *other == HOTEL_ROOM_IRI_HTTPS
	}
}
impl PartialEq<HotelRoomIri> for &str {
	fn eq(&self, other: &HotelRoomIri) -> bool {
		*self == HOTEL_ROOM_IRI_HTTP || *self == HOTEL_ROOM_IRI_HTTPS
	}
}
pub struct HotelRoomIriOrLabel;
impl PartialEq<&str> for HotelRoomIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HotelRoomIri || *other == HOTEL_ROOM_LABEL
	}
}
impl PartialEq<HotelRoomIriOrLabel> for &str {
	fn eq(&self, other: &HotelRoomIriOrLabel) -> bool {
		*self == HotelRoomIri || *self == HOTEL_ROOM_LABEL
	}
}
