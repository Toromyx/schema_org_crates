/// <https://schema.org/ChemicalSubstance>
pub const CHEMICAL_SUBSTANCE_IRI_HTTP: &str = "http://schema.org/ChemicalSubstance";
/// <https://schema.org/ChemicalSubstance>
pub const CHEMICAL_SUBSTANCE_IRI_HTTPS: &str = "https://schema.org/ChemicalSubstance";
/// <https://schema.org/ChemicalSubstance>
pub const CHEMICAL_SUBSTANCE_LABEL: &str = "ChemicalSubstance";
pub struct ChemicalSubstanceIri;
impl PartialEq<&str> for ChemicalSubstanceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHEMICAL_SUBSTANCE_IRI_HTTP || *other == CHEMICAL_SUBSTANCE_IRI_HTTPS
	}
}
impl PartialEq<ChemicalSubstanceIri> for &str {
	fn eq(&self, other: &ChemicalSubstanceIri) -> bool {
		*self == CHEMICAL_SUBSTANCE_IRI_HTTP || *self == CHEMICAL_SUBSTANCE_IRI_HTTPS
	}
}
pub struct ChemicalSubstanceIriOrLabel;
impl PartialEq<&str> for ChemicalSubstanceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChemicalSubstanceIri || *other == CHEMICAL_SUBSTANCE_LABEL
	}
}
impl PartialEq<ChemicalSubstanceIriOrLabel> for &str {
	fn eq(&self, other: &ChemicalSubstanceIriOrLabel) -> bool {
		*self == ChemicalSubstanceIri || *self == CHEMICAL_SUBSTANCE_LABEL
	}
}
