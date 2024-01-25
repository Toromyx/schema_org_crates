/// <https://schema.org/departureStation>
pub const DEPARTURE_STATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/departureStation";
/// <https://schema.org/departureStation>
pub const DEPARTURE_STATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/departureStation";
/// <https://schema.org/departureStation>
pub const DEPARTURE_STATION_PROPERTY_LABEL: &str = "departureStation";
pub struct DepartureStationPropertyIri;
impl PartialEq<&str> for DepartureStationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTURE_STATION_PROPERTY_IRI_HTTP
			|| *other == DEPARTURE_STATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepartureStationPropertyIri> for &str {
	fn eq(&self, other: &DepartureStationPropertyIri) -> bool {
		*self == DEPARTURE_STATION_PROPERTY_IRI_HTTP
			|| *self == DEPARTURE_STATION_PROPERTY_IRI_HTTPS
	}
}
pub struct DepartureStationPropertyIriOrLabel;
impl PartialEq<&str> for DepartureStationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartureStationPropertyIri || *other == DEPARTURE_STATION_PROPERTY_LABEL
	}
}
impl PartialEq<DepartureStationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepartureStationPropertyIriOrLabel) -> bool {
		*self == DepartureStationPropertyIri || *self == DEPARTURE_STATION_PROPERTY_LABEL
	}
}
