/// <https://schema.org/dateVehicleFirstRegistered>
pub const DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/dateVehicleFirstRegistered";
/// <https://schema.org/dateVehicleFirstRegistered>
pub const DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/dateVehicleFirstRegistered";
/// <https://schema.org/dateVehicleFirstRegistered>
pub const DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_LABEL: &str = "dateVehicleFirstRegistered";
pub struct DateVehicleFirstRegisteredPropertyIri;
impl PartialEq<&str> for DateVehicleFirstRegisteredPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_IRI_HTTP
			|| *other == DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DateVehicleFirstRegisteredPropertyIri> for &str {
	fn eq(&self, other: &DateVehicleFirstRegisteredPropertyIri) -> bool {
		*self == DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_IRI_HTTP
			|| *self == DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_IRI_HTTPS
	}
}
pub struct DateVehicleFirstRegisteredPropertyIriOrLabel;
impl PartialEq<&str> for DateVehicleFirstRegisteredPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateVehicleFirstRegisteredPropertyIri
			|| *other == DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_LABEL
	}
}
impl PartialEq<DateVehicleFirstRegisteredPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DateVehicleFirstRegisteredPropertyIriOrLabel) -> bool {
		*self == DateVehicleFirstRegisteredPropertyIri
			|| *self == DATE_VEHICLE_FIRST_REGISTERED_PROPERTY_LABEL
	}
}
