/// <https://schema.org/BodyMeasurementChest>
pub const BODY_MEASUREMENT_CHEST_IRI_HTTP: &str = "http://schema.org/BodyMeasurementChest";
/// <https://schema.org/BodyMeasurementChest>
pub const BODY_MEASUREMENT_CHEST_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementChest";
/// <https://schema.org/BodyMeasurementChest>
pub const BODY_MEASUREMENT_CHEST_LABEL: &str = "BodyMeasurementChest";
pub struct BodyMeasurementChestIri;
impl PartialEq<&str> for BodyMeasurementChestIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_CHEST_IRI_HTTP || *other == BODY_MEASUREMENT_CHEST_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementChestIri> for &str {
	fn eq(&self, other: &BodyMeasurementChestIri) -> bool {
		*self == BODY_MEASUREMENT_CHEST_IRI_HTTP || *self == BODY_MEASUREMENT_CHEST_IRI_HTTPS
	}
}
pub struct BodyMeasurementChestIriOrLabel;
impl PartialEq<&str> for BodyMeasurementChestIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementChestIri || *other == BODY_MEASUREMENT_CHEST_LABEL
	}
}
impl PartialEq<BodyMeasurementChestIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementChestIriOrLabel) -> bool {
		*self == BodyMeasurementChestIri || *self == BODY_MEASUREMENT_CHEST_LABEL
	}
}
