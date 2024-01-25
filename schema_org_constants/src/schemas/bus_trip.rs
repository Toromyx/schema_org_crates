/// <https://schema.org/BusTrip>
pub const BUS_TRIP_IRI_HTTP: &str = "http://schema.org/BusTrip";
/// <https://schema.org/BusTrip>
pub const BUS_TRIP_IRI_HTTPS: &str = "https://schema.org/BusTrip";
/// <https://schema.org/BusTrip>
pub const BUS_TRIP_LABEL: &str = "BusTrip";
pub struct BusTripIri;
impl PartialEq<&str> for BusTripIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUS_TRIP_IRI_HTTP || *other == BUS_TRIP_IRI_HTTPS
	}
}
impl PartialEq<BusTripIri> for &str {
	fn eq(&self, other: &BusTripIri) -> bool {
		*self == BUS_TRIP_IRI_HTTP || *self == BUS_TRIP_IRI_HTTPS
	}
}
pub struct BusTripIriOrLabel;
impl PartialEq<&str> for BusTripIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusTripIri || *other == BUS_TRIP_LABEL
	}
}
impl PartialEq<BusTripIriOrLabel> for &str {
	fn eq(&self, other: &BusTripIriOrLabel) -> bool {
		*self == BusTripIri || *self == BUS_TRIP_LABEL
	}
}
