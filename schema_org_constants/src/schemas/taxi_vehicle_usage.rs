/// <https://schema.org/TaxiVehicleUsage>
pub const TAXI_VEHICLE_USAGE_IRI_HTTP: &str = "http://schema.org/TaxiVehicleUsage";
/// <https://schema.org/TaxiVehicleUsage>
pub const TAXI_VEHICLE_USAGE_IRI_HTTPS: &str = "https://schema.org/TaxiVehicleUsage";
/// <https://schema.org/TaxiVehicleUsage>
pub const TAXI_VEHICLE_USAGE_LABEL: &str = "TaxiVehicleUsage";
pub struct TaxiVehicleUsageIri;
impl PartialEq<&str> for TaxiVehicleUsageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAXI_VEHICLE_USAGE_IRI_HTTP || *other == TAXI_VEHICLE_USAGE_IRI_HTTPS
	}
}
impl PartialEq<TaxiVehicleUsageIri> for &str {
	fn eq(&self, other: &TaxiVehicleUsageIri) -> bool {
		*self == TAXI_VEHICLE_USAGE_IRI_HTTP || *self == TAXI_VEHICLE_USAGE_IRI_HTTPS
	}
}
pub struct TaxiVehicleUsageIriOrLabel;
impl PartialEq<&str> for TaxiVehicleUsageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxiVehicleUsageIri || *other == TAXI_VEHICLE_USAGE_LABEL
	}
}
impl PartialEq<TaxiVehicleUsageIriOrLabel> for &str {
	fn eq(&self, other: &TaxiVehicleUsageIriOrLabel) -> bool {
		*self == TaxiVehicleUsageIri || *self == TAXI_VEHICLE_USAGE_LABEL
	}
}
