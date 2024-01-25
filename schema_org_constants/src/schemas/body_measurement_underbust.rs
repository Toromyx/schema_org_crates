/// <https://schema.org/BodyMeasurementUnderbust>
pub const BODY_MEASUREMENT_UNDERBUST_IRI_HTTP: &str = "http://schema.org/BodyMeasurementUnderbust";
/// <https://schema.org/BodyMeasurementUnderbust>
pub const BODY_MEASUREMENT_UNDERBUST_IRI_HTTPS: &str =
	"https://schema.org/BodyMeasurementUnderbust";
/// <https://schema.org/BodyMeasurementUnderbust>
pub const BODY_MEASUREMENT_UNDERBUST_LABEL: &str = "BodyMeasurementUnderbust";
pub struct BodyMeasurementUnderbustIri;
impl PartialEq<&str> for BodyMeasurementUnderbustIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_UNDERBUST_IRI_HTTP
			|| *other == BODY_MEASUREMENT_UNDERBUST_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementUnderbustIri> for &str {
	fn eq(&self, other: &BodyMeasurementUnderbustIri) -> bool {
		*self == BODY_MEASUREMENT_UNDERBUST_IRI_HTTP
			|| *self == BODY_MEASUREMENT_UNDERBUST_IRI_HTTPS
	}
}
pub struct BodyMeasurementUnderbustIriOrLabel;
impl PartialEq<&str> for BodyMeasurementUnderbustIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementUnderbustIri || *other == BODY_MEASUREMENT_UNDERBUST_LABEL
	}
}
impl PartialEq<BodyMeasurementUnderbustIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementUnderbustIriOrLabel) -> bool {
		*self == BodyMeasurementUnderbustIri || *self == BODY_MEASUREMENT_UNDERBUST_LABEL
	}
}
