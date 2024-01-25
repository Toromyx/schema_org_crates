/// <https://schema.org/BusStation>
pub const BUS_STATION_IRI_HTTP: &str = "http://schema.org/BusStation";
/// <https://schema.org/BusStation>
pub const BUS_STATION_IRI_HTTPS: &str = "https://schema.org/BusStation";
/// <https://schema.org/BusStation>
pub const BUS_STATION_LABEL: &str = "BusStation";
pub struct BusStationIri;
impl PartialEq<&str> for BusStationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUS_STATION_IRI_HTTP || *other == BUS_STATION_IRI_HTTPS
	}
}
impl PartialEq<BusStationIri> for &str {
	fn eq(&self, other: &BusStationIri) -> bool {
		*self == BUS_STATION_IRI_HTTP || *self == BUS_STATION_IRI_HTTPS
	}
}
pub struct BusStationIriOrLabel;
impl PartialEq<&str> for BusStationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusStationIri || *other == BUS_STATION_LABEL
	}
}
impl PartialEq<BusStationIriOrLabel> for &str {
	fn eq(&self, other: &BusStationIriOrLabel) -> bool {
		*self == BusStationIri || *self == BUS_STATION_LABEL
	}
}
