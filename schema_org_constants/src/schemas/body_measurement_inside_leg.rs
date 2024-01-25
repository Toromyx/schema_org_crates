/// <https://schema.org/BodyMeasurementInsideLeg>
pub const BODY_MEASUREMENT_INSIDE_LEG_IRI_HTTP: &str = "http://schema.org/BodyMeasurementInsideLeg";
/// <https://schema.org/BodyMeasurementInsideLeg>
pub const BODY_MEASUREMENT_INSIDE_LEG_IRI_HTTPS: &str =
	"https://schema.org/BodyMeasurementInsideLeg";
/// <https://schema.org/BodyMeasurementInsideLeg>
pub const BODY_MEASUREMENT_INSIDE_LEG_LABEL: &str = "BodyMeasurementInsideLeg";
pub struct BodyMeasurementInsideLegIri;
impl PartialEq<&str> for BodyMeasurementInsideLegIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_INSIDE_LEG_IRI_HTTP
			|| *other == BODY_MEASUREMENT_INSIDE_LEG_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementInsideLegIri> for &str {
	fn eq(&self, other: &BodyMeasurementInsideLegIri) -> bool {
		*self == BODY_MEASUREMENT_INSIDE_LEG_IRI_HTTP
			|| *self == BODY_MEASUREMENT_INSIDE_LEG_IRI_HTTPS
	}
}
pub struct BodyMeasurementInsideLegIriOrLabel;
impl PartialEq<&str> for BodyMeasurementInsideLegIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementInsideLegIri || *other == BODY_MEASUREMENT_INSIDE_LEG_LABEL
	}
}
impl PartialEq<BodyMeasurementInsideLegIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementInsideLegIriOrLabel) -> bool {
		*self == BodyMeasurementInsideLegIri || *self == BODY_MEASUREMENT_INSIDE_LEG_LABEL
	}
}
