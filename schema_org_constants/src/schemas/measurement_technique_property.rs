/// <https://schema.org/measurementTechnique>
pub const MEASUREMENT_TECHNIQUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/measurementTechnique";
/// <https://schema.org/measurementTechnique>
pub const MEASUREMENT_TECHNIQUE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/measurementTechnique";
/// <https://schema.org/measurementTechnique>
pub const MEASUREMENT_TECHNIQUE_PROPERTY_LABEL: &str = "measurementTechnique";
pub struct MeasurementTechniquePropertyIri;
impl PartialEq<&str> for MeasurementTechniquePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEASUREMENT_TECHNIQUE_PROPERTY_IRI_HTTP
			|| *other == MEASUREMENT_TECHNIQUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MeasurementTechniquePropertyIri> for &str {
	fn eq(&self, other: &MeasurementTechniquePropertyIri) -> bool {
		*self == MEASUREMENT_TECHNIQUE_PROPERTY_IRI_HTTP
			|| *self == MEASUREMENT_TECHNIQUE_PROPERTY_IRI_HTTPS
	}
}
pub struct MeasurementTechniquePropertyIriOrLabel;
impl PartialEq<&str> for MeasurementTechniquePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeasurementTechniquePropertyIri || *other == MEASUREMENT_TECHNIQUE_PROPERTY_LABEL
	}
}
impl PartialEq<MeasurementTechniquePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MeasurementTechniquePropertyIriOrLabel) -> bool {
		*self == MeasurementTechniquePropertyIri || *self == MEASUREMENT_TECHNIQUE_PROPERTY_LABEL
	}
}
