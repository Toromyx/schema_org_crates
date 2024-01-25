/// <https://schema.org/measurementMethod>
pub const MEASUREMENT_METHOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/measurementMethod";
/// <https://schema.org/measurementMethod>
pub const MEASUREMENT_METHOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/measurementMethod";
/// <https://schema.org/measurementMethod>
pub const MEASUREMENT_METHOD_PROPERTY_LABEL: &str = "measurementMethod";
pub struct MeasurementMethodPropertyIri;
impl PartialEq<&str> for MeasurementMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEASUREMENT_METHOD_PROPERTY_IRI_HTTP
			|| *other == MEASUREMENT_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MeasurementMethodPropertyIri> for &str {
	fn eq(&self, other: &MeasurementMethodPropertyIri) -> bool {
		*self == MEASUREMENT_METHOD_PROPERTY_IRI_HTTP
			|| *self == MEASUREMENT_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct MeasurementMethodPropertyIriOrLabel;
impl PartialEq<&str> for MeasurementMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeasurementMethodPropertyIri || *other == MEASUREMENT_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<MeasurementMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MeasurementMethodPropertyIriOrLabel) -> bool {
		*self == MeasurementMethodPropertyIri || *self == MEASUREMENT_METHOD_PROPERTY_LABEL
	}
}
