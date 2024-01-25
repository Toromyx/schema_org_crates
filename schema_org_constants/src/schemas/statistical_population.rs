/// <https://schema.org/StatisticalPopulation>
pub const STATISTICAL_POPULATION_IRI_HTTP: &str = "http://schema.org/StatisticalPopulation";
/// <https://schema.org/StatisticalPopulation>
pub const STATISTICAL_POPULATION_IRI_HTTPS: &str = "https://schema.org/StatisticalPopulation";
/// <https://schema.org/StatisticalPopulation>
pub const STATISTICAL_POPULATION_LABEL: &str = "StatisticalPopulation";
pub struct StatisticalPopulationIri;
impl PartialEq<&str> for StatisticalPopulationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STATISTICAL_POPULATION_IRI_HTTP || *other == STATISTICAL_POPULATION_IRI_HTTPS
	}
}
impl PartialEq<StatisticalPopulationIri> for &str {
	fn eq(&self, other: &StatisticalPopulationIri) -> bool {
		*self == STATISTICAL_POPULATION_IRI_HTTP || *self == STATISTICAL_POPULATION_IRI_HTTPS
	}
}
pub struct StatisticalPopulationIriOrLabel;
impl PartialEq<&str> for StatisticalPopulationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StatisticalPopulationIri || *other == STATISTICAL_POPULATION_LABEL
	}
}
impl PartialEq<StatisticalPopulationIriOrLabel> for &str {
	fn eq(&self, other: &StatisticalPopulationIriOrLabel) -> bool {
		*self == StatisticalPopulationIri || *self == STATISTICAL_POPULATION_LABEL
	}
}
