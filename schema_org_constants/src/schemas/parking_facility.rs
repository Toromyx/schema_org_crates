/// <https://schema.org/ParkingFacility>
pub const PARKING_FACILITY_IRI_HTTP: &str = "http://schema.org/ParkingFacility";
/// <https://schema.org/ParkingFacility>
pub const PARKING_FACILITY_IRI_HTTPS: &str = "https://schema.org/ParkingFacility";
/// <https://schema.org/ParkingFacility>
pub const PARKING_FACILITY_LABEL: &str = "ParkingFacility";
pub struct ParkingFacilityIri;
impl PartialEq<&str> for ParkingFacilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARKING_FACILITY_IRI_HTTP || *other == PARKING_FACILITY_IRI_HTTPS
	}
}
impl PartialEq<ParkingFacilityIri> for &str {
	fn eq(&self, other: &ParkingFacilityIri) -> bool {
		*self == PARKING_FACILITY_IRI_HTTP || *self == PARKING_FACILITY_IRI_HTTPS
	}
}
pub struct ParkingFacilityIriOrLabel;
impl PartialEq<&str> for ParkingFacilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParkingFacilityIri || *other == PARKING_FACILITY_LABEL
	}
}
impl PartialEq<ParkingFacilityIriOrLabel> for &str {
	fn eq(&self, other: &ParkingFacilityIriOrLabel) -> bool {
		*self == ParkingFacilityIri || *self == PARKING_FACILITY_LABEL
	}
}
