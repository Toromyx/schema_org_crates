/// <https://schema.org/measurementQualifier>
pub const MEASUREMENT_QUALIFIER_PROPERTY_IRI_HTTP: &str = "http://schema.org/measurementQualifier";
/// <https://schema.org/measurementQualifier>
pub const MEASUREMENT_QUALIFIER_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/measurementQualifier";
/// <https://schema.org/measurementQualifier>
pub const MEASUREMENT_QUALIFIER_PROPERTY_LABEL: &str = "measurementQualifier";
pub struct MeasurementQualifierPropertyIri;
impl PartialEq<&str> for MeasurementQualifierPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEASUREMENT_QUALIFIER_PROPERTY_IRI_HTTP
			|| *other == MEASUREMENT_QUALIFIER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MeasurementQualifierPropertyIri> for &str {
	fn eq(&self, other: &MeasurementQualifierPropertyIri) -> bool {
		*self == MEASUREMENT_QUALIFIER_PROPERTY_IRI_HTTP
			|| *self == MEASUREMENT_QUALIFIER_PROPERTY_IRI_HTTPS
	}
}
pub struct MeasurementQualifierPropertyIriOrLabel;
impl PartialEq<&str> for MeasurementQualifierPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeasurementQualifierPropertyIri || *other == MEASUREMENT_QUALIFIER_PROPERTY_LABEL
	}
}
impl PartialEq<MeasurementQualifierPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MeasurementQualifierPropertyIriOrLabel) -> bool {
		*self == MeasurementQualifierPropertyIri || *self == MEASUREMENT_QUALIFIER_PROPERTY_LABEL
	}
}
