/// <https://schema.org/populationType>
pub const POPULATION_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/populationType";
/// <https://schema.org/populationType>
pub const POPULATION_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/populationType";
/// <https://schema.org/populationType>
pub const POPULATION_TYPE_PROPERTY_LABEL: &str = "populationType";
pub struct PopulationTypePropertyIri;
impl PartialEq<&str> for PopulationTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POPULATION_TYPE_PROPERTY_IRI_HTTP || *other == POPULATION_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PopulationTypePropertyIri> for &str {
	fn eq(&self, other: &PopulationTypePropertyIri) -> bool {
		*self == POPULATION_TYPE_PROPERTY_IRI_HTTP || *self == POPULATION_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct PopulationTypePropertyIriOrLabel;
impl PartialEq<&str> for PopulationTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PopulationTypePropertyIri || *other == POPULATION_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<PopulationTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PopulationTypePropertyIriOrLabel) -> bool {
		*self == PopulationTypePropertyIri || *self == POPULATION_TYPE_PROPERTY_LABEL
	}
}
