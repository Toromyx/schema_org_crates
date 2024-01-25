/// <https://schema.org/mileageFromOdometer>
pub const MILEAGE_FROM_ODOMETER_PROPERTY_IRI_HTTP: &str = "http://schema.org/mileageFromOdometer";
/// <https://schema.org/mileageFromOdometer>
pub const MILEAGE_FROM_ODOMETER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mileageFromOdometer";
/// <https://schema.org/mileageFromOdometer>
pub const MILEAGE_FROM_ODOMETER_PROPERTY_LABEL: &str = "mileageFromOdometer";
pub struct MileageFromOdometerPropertyIri;
impl PartialEq<&str> for MileageFromOdometerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MILEAGE_FROM_ODOMETER_PROPERTY_IRI_HTTP
			|| *other == MILEAGE_FROM_ODOMETER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MileageFromOdometerPropertyIri> for &str {
	fn eq(&self, other: &MileageFromOdometerPropertyIri) -> bool {
		*self == MILEAGE_FROM_ODOMETER_PROPERTY_IRI_HTTP
			|| *self == MILEAGE_FROM_ODOMETER_PROPERTY_IRI_HTTPS
	}
}
pub struct MileageFromOdometerPropertyIriOrLabel;
impl PartialEq<&str> for MileageFromOdometerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MileageFromOdometerPropertyIri || *other == MILEAGE_FROM_ODOMETER_PROPERTY_LABEL
	}
}
impl PartialEq<MileageFromOdometerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MileageFromOdometerPropertyIriOrLabel) -> bool {
		*self == MileageFromOdometerPropertyIri || *self == MILEAGE_FROM_ODOMETER_PROPERTY_LABEL
	}
}
