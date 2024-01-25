/// <https://schema.org/CableOrSatelliteService>
pub const CABLE_OR_SATELLITE_SERVICE_IRI_HTTP: &str = "http://schema.org/CableOrSatelliteService";
/// <https://schema.org/CableOrSatelliteService>
pub const CABLE_OR_SATELLITE_SERVICE_IRI_HTTPS: &str = "https://schema.org/CableOrSatelliteService";
/// <https://schema.org/CableOrSatelliteService>
pub const CABLE_OR_SATELLITE_SERVICE_LABEL: &str = "CableOrSatelliteService";
pub struct CableOrSatelliteServiceIri;
impl PartialEq<&str> for CableOrSatelliteServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CABLE_OR_SATELLITE_SERVICE_IRI_HTTP
			|| *other == CABLE_OR_SATELLITE_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<CableOrSatelliteServiceIri> for &str {
	fn eq(&self, other: &CableOrSatelliteServiceIri) -> bool {
		*self == CABLE_OR_SATELLITE_SERVICE_IRI_HTTP
			|| *self == CABLE_OR_SATELLITE_SERVICE_IRI_HTTPS
	}
}
pub struct CableOrSatelliteServiceIriOrLabel;
impl PartialEq<&str> for CableOrSatelliteServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CableOrSatelliteServiceIri || *other == CABLE_OR_SATELLITE_SERVICE_LABEL
	}
}
impl PartialEq<CableOrSatelliteServiceIriOrLabel> for &str {
	fn eq(&self, other: &CableOrSatelliteServiceIriOrLabel) -> bool {
		*self == CableOrSatelliteServiceIri || *self == CABLE_OR_SATELLITE_SERVICE_LABEL
	}
}
