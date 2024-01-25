/// <https://schema.org/FireStation>
pub const FIRE_STATION_IRI_HTTP: &str = "http://schema.org/FireStation";
/// <https://schema.org/FireStation>
pub const FIRE_STATION_IRI_HTTPS: &str = "https://schema.org/FireStation";
/// <https://schema.org/FireStation>
pub const FIRE_STATION_LABEL: &str = "FireStation";
pub struct FireStationIri;
impl PartialEq<&str> for FireStationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FIRE_STATION_IRI_HTTP || *other == FIRE_STATION_IRI_HTTPS
	}
}
impl PartialEq<FireStationIri> for &str {
	fn eq(&self, other: &FireStationIri) -> bool {
		*self == FIRE_STATION_IRI_HTTP || *self == FIRE_STATION_IRI_HTTPS
	}
}
pub struct FireStationIriOrLabel;
impl PartialEq<&str> for FireStationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FireStationIri || *other == FIRE_STATION_LABEL
	}
}
impl PartialEq<FireStationIriOrLabel> for &str {
	fn eq(&self, other: &FireStationIriOrLabel) -> bool {
		*self == FireStationIri || *self == FIRE_STATION_LABEL
	}
}
