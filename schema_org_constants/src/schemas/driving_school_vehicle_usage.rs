/// <https://schema.org/DrivingSchoolVehicleUsage>
pub const DRIVING_SCHOOL_VEHICLE_USAGE_IRI_HTTP: &str =
	"http://schema.org/DrivingSchoolVehicleUsage";
/// <https://schema.org/DrivingSchoolVehicleUsage>
pub const DRIVING_SCHOOL_VEHICLE_USAGE_IRI_HTTPS: &str =
	"https://schema.org/DrivingSchoolVehicleUsage";
/// <https://schema.org/DrivingSchoolVehicleUsage>
pub const DRIVING_SCHOOL_VEHICLE_USAGE_LABEL: &str = "DrivingSchoolVehicleUsage";
pub struct DrivingSchoolVehicleUsageIri;
impl PartialEq<&str> for DrivingSchoolVehicleUsageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRIVING_SCHOOL_VEHICLE_USAGE_IRI_HTTP
			|| *other == DRIVING_SCHOOL_VEHICLE_USAGE_IRI_HTTPS
	}
}
impl PartialEq<DrivingSchoolVehicleUsageIri> for &str {
	fn eq(&self, other: &DrivingSchoolVehicleUsageIri) -> bool {
		*self == DRIVING_SCHOOL_VEHICLE_USAGE_IRI_HTTP
			|| *self == DRIVING_SCHOOL_VEHICLE_USAGE_IRI_HTTPS
	}
}
pub struct DrivingSchoolVehicleUsageIriOrLabel;
impl PartialEq<&str> for DrivingSchoolVehicleUsageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrivingSchoolVehicleUsageIri || *other == DRIVING_SCHOOL_VEHICLE_USAGE_LABEL
	}
}
impl PartialEq<DrivingSchoolVehicleUsageIriOrLabel> for &str {
	fn eq(&self, other: &DrivingSchoolVehicleUsageIriOrLabel) -> bool {
		*self == DrivingSchoolVehicleUsageIri || *self == DRIVING_SCHOOL_VEHICLE_USAGE_LABEL
	}
}
