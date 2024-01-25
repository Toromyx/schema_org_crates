/// <https://schema.org/molecularFormula>
pub const MOLECULAR_FORMULA_PROPERTY_IRI_HTTP: &str = "http://schema.org/molecularFormula";
/// <https://schema.org/molecularFormula>
pub const MOLECULAR_FORMULA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/molecularFormula";
/// <https://schema.org/molecularFormula>
pub const MOLECULAR_FORMULA_PROPERTY_LABEL: &str = "molecularFormula";
pub struct MolecularFormulaPropertyIri;
impl PartialEq<&str> for MolecularFormulaPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOLECULAR_FORMULA_PROPERTY_IRI_HTTP
			|| *other == MOLECULAR_FORMULA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MolecularFormulaPropertyIri> for &str {
	fn eq(&self, other: &MolecularFormulaPropertyIri) -> bool {
		*self == MOLECULAR_FORMULA_PROPERTY_IRI_HTTP
			|| *self == MOLECULAR_FORMULA_PROPERTY_IRI_HTTPS
	}
}
pub struct MolecularFormulaPropertyIriOrLabel;
impl PartialEq<&str> for MolecularFormulaPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MolecularFormulaPropertyIri || *other == MOLECULAR_FORMULA_PROPERTY_LABEL
	}
}
impl PartialEq<MolecularFormulaPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MolecularFormulaPropertyIriOrLabel) -> bool {
		*self == MolecularFormulaPropertyIri || *self == MOLECULAR_FORMULA_PROPERTY_LABEL
	}
}
