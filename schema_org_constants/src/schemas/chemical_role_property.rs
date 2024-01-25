/// <https://schema.org/chemicalRole>
pub const CHEMICAL_ROLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/chemicalRole";
/// <https://schema.org/chemicalRole>
pub const CHEMICAL_ROLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/chemicalRole";
/// <https://schema.org/chemicalRole>
pub const CHEMICAL_ROLE_PROPERTY_LABEL: &str = "chemicalRole";
pub struct ChemicalRolePropertyIri;
impl PartialEq<&str> for ChemicalRolePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHEMICAL_ROLE_PROPERTY_IRI_HTTP || *other == CHEMICAL_ROLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ChemicalRolePropertyIri> for &str {
	fn eq(&self, other: &ChemicalRolePropertyIri) -> bool {
		*self == CHEMICAL_ROLE_PROPERTY_IRI_HTTP || *self == CHEMICAL_ROLE_PROPERTY_IRI_HTTPS
	}
}
pub struct ChemicalRolePropertyIriOrLabel;
impl PartialEq<&str> for ChemicalRolePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChemicalRolePropertyIri || *other == CHEMICAL_ROLE_PROPERTY_LABEL
	}
}
impl PartialEq<ChemicalRolePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ChemicalRolePropertyIriOrLabel) -> bool {
		*self == ChemicalRolePropertyIri || *self == CHEMICAL_ROLE_PROPERTY_LABEL
	}
}
