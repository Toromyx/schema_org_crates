/// <https://schema.org/ParkingMap>
pub const PARKING_MAP_IRI_HTTP: &str = "http://schema.org/ParkingMap";
/// <https://schema.org/ParkingMap>
pub const PARKING_MAP_IRI_HTTPS: &str = "https://schema.org/ParkingMap";
/// <https://schema.org/ParkingMap>
pub const PARKING_MAP_LABEL: &str = "ParkingMap";
pub struct ParkingMapIri;
impl PartialEq<&str> for ParkingMapIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARKING_MAP_IRI_HTTP || *other == PARKING_MAP_IRI_HTTPS
	}
}
impl PartialEq<ParkingMapIri> for &str {
	fn eq(&self, other: &ParkingMapIri) -> bool {
		*self == PARKING_MAP_IRI_HTTP || *self == PARKING_MAP_IRI_HTTPS
	}
}
pub struct ParkingMapIriOrLabel;
impl PartialEq<&str> for ParkingMapIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParkingMapIri || *other == PARKING_MAP_LABEL
	}
}
impl PartialEq<ParkingMapIriOrLabel> for &str {
	fn eq(&self, other: &ParkingMapIriOrLabel) -> bool {
		*self == ParkingMapIri || *self == PARKING_MAP_LABEL
	}
}
