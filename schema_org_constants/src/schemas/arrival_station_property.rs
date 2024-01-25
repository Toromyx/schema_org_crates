/// <https://schema.org/arrivalStation>
pub const ARRIVAL_STATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/arrivalStation";
/// <https://schema.org/arrivalStation>
pub const ARRIVAL_STATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arrivalStation";
/// <https://schema.org/arrivalStation>
pub const ARRIVAL_STATION_PROPERTY_LABEL: &str = "arrivalStation";
pub struct ArrivalStationPropertyIri;
impl PartialEq<&str> for ArrivalStationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVAL_STATION_PROPERTY_IRI_HTTP || *other == ARRIVAL_STATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArrivalStationPropertyIri> for &str {
	fn eq(&self, other: &ArrivalStationPropertyIri) -> bool {
		*self == ARRIVAL_STATION_PROPERTY_IRI_HTTP || *self == ARRIVAL_STATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ArrivalStationPropertyIriOrLabel;
impl PartialEq<&str> for ArrivalStationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArrivalStationPropertyIri || *other == ARRIVAL_STATION_PROPERTY_LABEL
	}
}
impl PartialEq<ArrivalStationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArrivalStationPropertyIriOrLabel) -> bool {
		*self == ArrivalStationPropertyIri || *self == ARRIVAL_STATION_PROPERTY_LABEL
	}
}
