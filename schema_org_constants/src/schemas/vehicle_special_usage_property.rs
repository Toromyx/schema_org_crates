/// <https://schema.org/vehicleSpecialUsage>
pub const VEHICLE_SPECIAL_USAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/vehicleSpecialUsage";
/// <https://schema.org/vehicleSpecialUsage>
pub const VEHICLE_SPECIAL_USAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/vehicleSpecialUsage";
/// <https://schema.org/vehicleSpecialUsage>
pub const VEHICLE_SPECIAL_USAGE_PROPERTY_LABEL: &str = "vehicleSpecialUsage";
pub struct VehicleSpecialUsagePropertyIri;
impl PartialEq<&str> for VehicleSpecialUsagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_SPECIAL_USAGE_PROPERTY_IRI_HTTP
			|| *other == VEHICLE_SPECIAL_USAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleSpecialUsagePropertyIri> for &str {
	fn eq(&self, other: &VehicleSpecialUsagePropertyIri) -> bool {
		*self == VEHICLE_SPECIAL_USAGE_PROPERTY_IRI_HTTP
			|| *self == VEHICLE_SPECIAL_USAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleSpecialUsagePropertyIriOrLabel;
impl PartialEq<&str> for VehicleSpecialUsagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleSpecialUsagePropertyIri || *other == VEHICLE_SPECIAL_USAGE_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleSpecialUsagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleSpecialUsagePropertyIriOrLabel) -> bool {
		*self == VehicleSpecialUsagePropertyIri || *self == VEHICLE_SPECIAL_USAGE_PROPERTY_LABEL
	}
}
