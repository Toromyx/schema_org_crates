/// <https://schema.org/PoliceStation>
pub const POLICE_STATION_IRI_HTTP: &str = "http://schema.org/PoliceStation";
/// <https://schema.org/PoliceStation>
pub const POLICE_STATION_IRI_HTTPS: &str = "https://schema.org/PoliceStation";
/// <https://schema.org/PoliceStation>
pub const POLICE_STATION_LABEL: &str = "PoliceStation";
pub struct PoliceStationIri;
impl PartialEq<&str> for PoliceStationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POLICE_STATION_IRI_HTTP || *other == POLICE_STATION_IRI_HTTPS
	}
}
impl PartialEq<PoliceStationIri> for &str {
	fn eq(&self, other: &PoliceStationIri) -> bool {
		*self == POLICE_STATION_IRI_HTTP || *self == POLICE_STATION_IRI_HTTPS
	}
}
pub struct PoliceStationIriOrLabel;
impl PartialEq<&str> for PoliceStationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PoliceStationIri || *other == POLICE_STATION_LABEL
	}
}
impl PartialEq<PoliceStationIriOrLabel> for &str {
	fn eq(&self, other: &PoliceStationIriOrLabel) -> bool {
		*self == PoliceStationIri || *self == POLICE_STATION_LABEL
	}
}
