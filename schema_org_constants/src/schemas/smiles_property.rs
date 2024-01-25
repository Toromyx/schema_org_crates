/// <https://schema.org/smiles>
pub const SMILES_PROPERTY_IRI_HTTP: &str = "http://schema.org/smiles";
/// <https://schema.org/smiles>
pub const SMILES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/smiles";
/// <https://schema.org/smiles>
pub const SMILES_PROPERTY_LABEL: &str = "smiles";
pub struct SmilesPropertyIri;
impl PartialEq<&str> for SmilesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SMILES_PROPERTY_IRI_HTTP || *other == SMILES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SmilesPropertyIri> for &str {
	fn eq(&self, other: &SmilesPropertyIri) -> bool {
		*self == SMILES_PROPERTY_IRI_HTTP || *self == SMILES_PROPERTY_IRI_HTTPS
	}
}
pub struct SmilesPropertyIriOrLabel;
impl PartialEq<&str> for SmilesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SmilesPropertyIri || *other == SMILES_PROPERTY_LABEL
	}
}
impl PartialEq<SmilesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SmilesPropertyIriOrLabel) -> bool {
		*self == SmilesPropertyIri || *self == SMILES_PROPERTY_LABEL
	}
}
