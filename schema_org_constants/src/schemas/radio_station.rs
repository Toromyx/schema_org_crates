/// <https://schema.org/RadioStation>
pub const RADIO_STATION_IRI_HTTP: &str = "http://schema.org/RadioStation";
/// <https://schema.org/RadioStation>
pub const RADIO_STATION_IRI_HTTPS: &str = "https://schema.org/RadioStation";
/// <https://schema.org/RadioStation>
pub const RADIO_STATION_LABEL: &str = "RadioStation";
pub struct RadioStationIri;
impl PartialEq<&str> for RadioStationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIO_STATION_IRI_HTTP || *other == RADIO_STATION_IRI_HTTPS
	}
}
impl PartialEq<RadioStationIri> for &str {
	fn eq(&self, other: &RadioStationIri) -> bool {
		*self == RADIO_STATION_IRI_HTTP || *self == RADIO_STATION_IRI_HTTPS
	}
}
pub struct RadioStationIriOrLabel;
impl PartialEq<&str> for RadioStationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadioStationIri || *other == RADIO_STATION_LABEL
	}
}
impl PartialEq<RadioStationIriOrLabel> for &str {
	fn eq(&self, other: &RadioStationIriOrLabel) -> bool {
		*self == RadioStationIri || *self == RADIO_STATION_LABEL
	}
}
