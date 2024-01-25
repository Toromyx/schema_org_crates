/// <https://schema.org/RentalVehicleUsage>
pub const RENTAL_VEHICLE_USAGE_IRI_HTTP: &str = "http://schema.org/RentalVehicleUsage";
/// <https://schema.org/RentalVehicleUsage>
pub const RENTAL_VEHICLE_USAGE_IRI_HTTPS: &str = "https://schema.org/RentalVehicleUsage";
/// <https://schema.org/RentalVehicleUsage>
pub const RENTAL_VEHICLE_USAGE_LABEL: &str = "RentalVehicleUsage";
pub struct RentalVehicleUsageIri;
impl PartialEq<&str> for RentalVehicleUsageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RENTAL_VEHICLE_USAGE_IRI_HTTP || *other == RENTAL_VEHICLE_USAGE_IRI_HTTPS
	}
}
impl PartialEq<RentalVehicleUsageIri> for &str {
	fn eq(&self, other: &RentalVehicleUsageIri) -> bool {
		*self == RENTAL_VEHICLE_USAGE_IRI_HTTP || *self == RENTAL_VEHICLE_USAGE_IRI_HTTPS
	}
}
pub struct RentalVehicleUsageIriOrLabel;
impl PartialEq<&str> for RentalVehicleUsageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RentalVehicleUsageIri || *other == RENTAL_VEHICLE_USAGE_LABEL
	}
}
impl PartialEq<RentalVehicleUsageIriOrLabel> for &str {
	fn eq(&self, other: &RentalVehicleUsageIriOrLabel) -> bool {
		*self == RentalVehicleUsageIri || *self == RENTAL_VEHICLE_USAGE_LABEL
	}
}
