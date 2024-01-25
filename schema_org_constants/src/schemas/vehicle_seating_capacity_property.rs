/// <https://schema.org/vehicleSeatingCapacity>
pub const VEHICLE_SEATING_CAPACITY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/vehicleSeatingCapacity";
/// <https://schema.org/vehicleSeatingCapacity>
pub const VEHICLE_SEATING_CAPACITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/vehicleSeatingCapacity";
/// <https://schema.org/vehicleSeatingCapacity>
pub const VEHICLE_SEATING_CAPACITY_PROPERTY_LABEL: &str = "vehicleSeatingCapacity";
pub struct VehicleSeatingCapacityPropertyIri;
impl PartialEq<&str> for VehicleSeatingCapacityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_SEATING_CAPACITY_PROPERTY_IRI_HTTP
			|| *other == VEHICLE_SEATING_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleSeatingCapacityPropertyIri> for &str {
	fn eq(&self, other: &VehicleSeatingCapacityPropertyIri) -> bool {
		*self == VEHICLE_SEATING_CAPACITY_PROPERTY_IRI_HTTP
			|| *self == VEHICLE_SEATING_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleSeatingCapacityPropertyIriOrLabel;
impl PartialEq<&str> for VehicleSeatingCapacityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleSeatingCapacityPropertyIri
			|| *other == VEHICLE_SEATING_CAPACITY_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleSeatingCapacityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleSeatingCapacityPropertyIriOrLabel) -> bool {
		*self == VehicleSeatingCapacityPropertyIri
			|| *self == VEHICLE_SEATING_CAPACITY_PROPERTY_LABEL
	}
}
