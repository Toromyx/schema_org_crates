/// <https://schema.org/numberOfRooms>
pub const NUMBER_OF_ROOMS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfRooms";
/// <https://schema.org/numberOfRooms>
pub const NUMBER_OF_ROOMS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfRooms";
/// <https://schema.org/numberOfRooms>
pub const NUMBER_OF_ROOMS_PROPERTY_LABEL: &str = "numberOfRooms";
pub struct NumberOfRoomsPropertyIri;
impl PartialEq<&str> for NumberOfRoomsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_ROOMS_PROPERTY_IRI_HTTP || *other == NUMBER_OF_ROOMS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfRoomsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfRoomsPropertyIri) -> bool {
		*self == NUMBER_OF_ROOMS_PROPERTY_IRI_HTTP || *self == NUMBER_OF_ROOMS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfRoomsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfRoomsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfRoomsPropertyIri || *other == NUMBER_OF_ROOMS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfRoomsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfRoomsPropertyIriOrLabel) -> bool {
		*self == NumberOfRoomsPropertyIri || *self == NUMBER_OF_ROOMS_PROPERTY_LABEL
	}
}
