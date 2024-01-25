/// <https://schema.org/targetPopulation>
pub const TARGET_POPULATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/targetPopulation";
/// <https://schema.org/targetPopulation>
pub const TARGET_POPULATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/targetPopulation";
/// <https://schema.org/targetPopulation>
pub const TARGET_POPULATION_PROPERTY_LABEL: &str = "targetPopulation";
pub struct TargetPopulationPropertyIri;
impl PartialEq<&str> for TargetPopulationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TARGET_POPULATION_PROPERTY_IRI_HTTP
			|| *other == TARGET_POPULATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TargetPopulationPropertyIri> for &str {
	fn eq(&self, other: &TargetPopulationPropertyIri) -> bool {
		*self == TARGET_POPULATION_PROPERTY_IRI_HTTP
			|| *self == TARGET_POPULATION_PROPERTY_IRI_HTTPS
	}
}
pub struct TargetPopulationPropertyIriOrLabel;
impl PartialEq<&str> for TargetPopulationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TargetPopulationPropertyIri || *other == TARGET_POPULATION_PROPERTY_LABEL
	}
}
impl PartialEq<TargetPopulationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TargetPopulationPropertyIriOrLabel) -> bool {
		*self == TargetPopulationPropertyIri || *self == TARGET_POPULATION_PROPERTY_LABEL
	}
}
