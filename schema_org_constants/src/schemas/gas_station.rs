/// <https://schema.org/GasStation>
pub const GAS_STATION_IRI_HTTP: &str = "http://schema.org/GasStation";
/// <https://schema.org/GasStation>
pub const GAS_STATION_IRI_HTTPS: &str = "https://schema.org/GasStation";
/// <https://schema.org/GasStation>
pub const GAS_STATION_LABEL: &str = "GasStation";
pub struct GasStationIri;
impl PartialEq<&str> for GasStationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAS_STATION_IRI_HTTP || *other == GAS_STATION_IRI_HTTPS
	}
}
impl PartialEq<GasStationIri> for &str {
	fn eq(&self, other: &GasStationIri) -> bool {
		*self == GAS_STATION_IRI_HTTP || *self == GAS_STATION_IRI_HTTPS
	}
}
pub struct GasStationIriOrLabel;
impl PartialEq<&str> for GasStationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GasStationIri || *other == GAS_STATION_LABEL
	}
}
impl PartialEq<GasStationIriOrLabel> for &str {
	fn eq(&self, other: &GasStationIriOrLabel) -> bool {
		*self == GasStationIri || *self == GAS_STATION_LABEL
	}
}
