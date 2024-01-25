/// <https://schema.org/hasMeasurement>
pub const HAS_MEASUREMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasMeasurement";
/// <https://schema.org/hasMeasurement>
pub const HAS_MEASUREMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasMeasurement";
/// <https://schema.org/hasMeasurement>
pub const HAS_MEASUREMENT_PROPERTY_LABEL: &str = "hasMeasurement";
pub struct HasMeasurementPropertyIri;
impl PartialEq<&str> for HasMeasurementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_MEASUREMENT_PROPERTY_IRI_HTTP || *other == HAS_MEASUREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasMeasurementPropertyIri> for &str {
	fn eq(&self, other: &HasMeasurementPropertyIri) -> bool {
		*self == HAS_MEASUREMENT_PROPERTY_IRI_HTTP || *self == HAS_MEASUREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct HasMeasurementPropertyIriOrLabel;
impl PartialEq<&str> for HasMeasurementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasMeasurementPropertyIri || *other == HAS_MEASUREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<HasMeasurementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasMeasurementPropertyIriOrLabel) -> bool {
		*self == HasMeasurementPropertyIri || *self == HAS_MEASUREMENT_PROPERTY_LABEL
	}
}
