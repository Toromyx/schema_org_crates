/// <https://schema.org/chemicalComposition>
pub const CHEMICAL_COMPOSITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/chemicalComposition";
/// <https://schema.org/chemicalComposition>
pub const CHEMICAL_COMPOSITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/chemicalComposition";
/// <https://schema.org/chemicalComposition>
pub const CHEMICAL_COMPOSITION_PROPERTY_LABEL: &str = "chemicalComposition";
pub struct ChemicalCompositionPropertyIri;
impl PartialEq<&str> for ChemicalCompositionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHEMICAL_COMPOSITION_PROPERTY_IRI_HTTP
			|| *other == CHEMICAL_COMPOSITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ChemicalCompositionPropertyIri> for &str {
	fn eq(&self, other: &ChemicalCompositionPropertyIri) -> bool {
		*self == CHEMICAL_COMPOSITION_PROPERTY_IRI_HTTP
			|| *self == CHEMICAL_COMPOSITION_PROPERTY_IRI_HTTPS
	}
}
pub struct ChemicalCompositionPropertyIriOrLabel;
impl PartialEq<&str> for ChemicalCompositionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChemicalCompositionPropertyIri || *other == CHEMICAL_COMPOSITION_PROPERTY_LABEL
	}
}
impl PartialEq<ChemicalCompositionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ChemicalCompositionPropertyIriOrLabel) -> bool {
		*self == ChemicalCompositionPropertyIri || *self == CHEMICAL_COMPOSITION_PROPERTY_LABEL
	}
}
