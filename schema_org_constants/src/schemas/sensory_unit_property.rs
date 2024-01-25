/// <https://schema.org/sensoryUnit>
pub const SENSORY_UNIT_PROPERTY_IRI_HTTP: &str = "http://schema.org/sensoryUnit";
/// <https://schema.org/sensoryUnit>
pub const SENSORY_UNIT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sensoryUnit";
/// <https://schema.org/sensoryUnit>
pub const SENSORY_UNIT_PROPERTY_LABEL: &str = "sensoryUnit";
pub struct SensoryUnitPropertyIri;
impl PartialEq<&str> for SensoryUnitPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SENSORY_UNIT_PROPERTY_IRI_HTTP || *other == SENSORY_UNIT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SensoryUnitPropertyIri> for &str {
	fn eq(&self, other: &SensoryUnitPropertyIri) -> bool {
		*self == SENSORY_UNIT_PROPERTY_IRI_HTTP || *self == SENSORY_UNIT_PROPERTY_IRI_HTTPS
	}
}
pub struct SensoryUnitPropertyIriOrLabel;
impl PartialEq<&str> for SensoryUnitPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SensoryUnitPropertyIri || *other == SENSORY_UNIT_PROPERTY_LABEL
	}
}
impl PartialEq<SensoryUnitPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SensoryUnitPropertyIriOrLabel) -> bool {
		*self == SensoryUnitPropertyIri || *self == SENSORY_UNIT_PROPERTY_LABEL
	}
}
