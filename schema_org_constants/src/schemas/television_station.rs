/// <https://schema.org/TelevisionStation>
pub const TELEVISION_STATION_IRI_HTTP: &str = "http://schema.org/TelevisionStation";
/// <https://schema.org/TelevisionStation>
pub const TELEVISION_STATION_IRI_HTTPS: &str = "https://schema.org/TelevisionStation";
/// <https://schema.org/TelevisionStation>
pub const TELEVISION_STATION_LABEL: &str = "TelevisionStation";
pub struct TelevisionStationIri;
impl PartialEq<&str> for TelevisionStationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TELEVISION_STATION_IRI_HTTP || *other == TELEVISION_STATION_IRI_HTTPS
	}
}
impl PartialEq<TelevisionStationIri> for &str {
	fn eq(&self, other: &TelevisionStationIri) -> bool {
		*self == TELEVISION_STATION_IRI_HTTP || *self == TELEVISION_STATION_IRI_HTTPS
	}
}
pub struct TelevisionStationIriOrLabel;
impl PartialEq<&str> for TelevisionStationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TelevisionStationIri || *other == TELEVISION_STATION_LABEL
	}
}
impl PartialEq<TelevisionStationIriOrLabel> for &str {
	fn eq(&self, other: &TelevisionStationIriOrLabel) -> bool {
		*self == TelevisionStationIri || *self == TELEVISION_STATION_LABEL
	}
}
