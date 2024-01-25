/// <https://schema.org/measurementDenominator>
pub const MEASUREMENT_DENOMINATOR_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/measurementDenominator";
/// <https://schema.org/measurementDenominator>
pub const MEASUREMENT_DENOMINATOR_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/measurementDenominator";
/// <https://schema.org/measurementDenominator>
pub const MEASUREMENT_DENOMINATOR_PROPERTY_LABEL: &str = "measurementDenominator";
pub struct MeasurementDenominatorPropertyIri;
impl PartialEq<&str> for MeasurementDenominatorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEASUREMENT_DENOMINATOR_PROPERTY_IRI_HTTP
			|| *other == MEASUREMENT_DENOMINATOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MeasurementDenominatorPropertyIri> for &str {
	fn eq(&self, other: &MeasurementDenominatorPropertyIri) -> bool {
		*self == MEASUREMENT_DENOMINATOR_PROPERTY_IRI_HTTP
			|| *self == MEASUREMENT_DENOMINATOR_PROPERTY_IRI_HTTPS
	}
}
pub struct MeasurementDenominatorPropertyIriOrLabel;
impl PartialEq<&str> for MeasurementDenominatorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeasurementDenominatorPropertyIri
			|| *other == MEASUREMENT_DENOMINATOR_PROPERTY_LABEL
	}
}
impl PartialEq<MeasurementDenominatorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MeasurementDenominatorPropertyIriOrLabel) -> bool {
		*self == MeasurementDenominatorPropertyIri
			|| *self == MEASUREMENT_DENOMINATOR_PROPERTY_LABEL
	}
}
