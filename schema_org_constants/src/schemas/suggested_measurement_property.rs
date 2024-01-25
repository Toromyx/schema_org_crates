/// <https://schema.org/suggestedMeasurement>
pub const SUGGESTED_MEASUREMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/suggestedMeasurement";
/// <https://schema.org/suggestedMeasurement>
pub const SUGGESTED_MEASUREMENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/suggestedMeasurement";
/// <https://schema.org/suggestedMeasurement>
pub const SUGGESTED_MEASUREMENT_PROPERTY_LABEL: &str = "suggestedMeasurement";
pub struct SuggestedMeasurementPropertyIri;
impl PartialEq<&str> for SuggestedMeasurementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUGGESTED_MEASUREMENT_PROPERTY_IRI_HTTP
			|| *other == SUGGESTED_MEASUREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuggestedMeasurementPropertyIri> for &str {
	fn eq(&self, other: &SuggestedMeasurementPropertyIri) -> bool {
		*self == SUGGESTED_MEASUREMENT_PROPERTY_IRI_HTTP
			|| *self == SUGGESTED_MEASUREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SuggestedMeasurementPropertyIriOrLabel;
impl PartialEq<&str> for SuggestedMeasurementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuggestedMeasurementPropertyIri || *other == SUGGESTED_MEASUREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<SuggestedMeasurementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuggestedMeasurementPropertyIriOrLabel) -> bool {
		*self == SuggestedMeasurementPropertyIri || *self == SUGGESTED_MEASUREMENT_PROPERTY_LABEL
	}
}
