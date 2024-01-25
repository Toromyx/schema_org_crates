/// <https://schema.org/SubwayStation>
pub const SUBWAY_STATION_IRI_HTTP: &str = "http://schema.org/SubwayStation";
/// <https://schema.org/SubwayStation>
pub const SUBWAY_STATION_IRI_HTTPS: &str = "https://schema.org/SubwayStation";
/// <https://schema.org/SubwayStation>
pub const SUBWAY_STATION_LABEL: &str = "SubwayStation";
pub struct SubwayStationIri;
impl PartialEq<&str> for SubwayStationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUBWAY_STATION_IRI_HTTP || *other == SUBWAY_STATION_IRI_HTTPS
	}
}
impl PartialEq<SubwayStationIri> for &str {
	fn eq(&self, other: &SubwayStationIri) -> bool {
		*self == SUBWAY_STATION_IRI_HTTP || *self == SUBWAY_STATION_IRI_HTTPS
	}
}
pub struct SubwayStationIriOrLabel;
impl PartialEq<&str> for SubwayStationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubwayStationIri || *other == SUBWAY_STATION_LABEL
	}
}
impl PartialEq<SubwayStationIriOrLabel> for &str {
	fn eq(&self, other: &SubwayStationIriOrLabel) -> bool {
		*self == SubwayStationIri || *self == SUBWAY_STATION_LABEL
	}
}
