/// <https://schema.org/MolecularEntity>
pub const MOLECULAR_ENTITY_IRI_HTTP: &str = "http://schema.org/MolecularEntity";
/// <https://schema.org/MolecularEntity>
pub const MOLECULAR_ENTITY_IRI_HTTPS: &str = "https://schema.org/MolecularEntity";
/// <https://schema.org/MolecularEntity>
pub const MOLECULAR_ENTITY_LABEL: &str = "MolecularEntity";
pub struct MolecularEntityIri;
impl PartialEq<&str> for MolecularEntityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOLECULAR_ENTITY_IRI_HTTP || *other == MOLECULAR_ENTITY_IRI_HTTPS
	}
}
impl PartialEq<MolecularEntityIri> for &str {
	fn eq(&self, other: &MolecularEntityIri) -> bool {
		*self == MOLECULAR_ENTITY_IRI_HTTP || *self == MOLECULAR_ENTITY_IRI_HTTPS
	}
}
pub struct MolecularEntityIriOrLabel;
impl PartialEq<&str> for MolecularEntityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MolecularEntityIri || *other == MOLECULAR_ENTITY_LABEL
	}
}
impl PartialEq<MolecularEntityIriOrLabel> for &str {
	fn eq(&self, other: &MolecularEntityIriOrLabel) -> bool {
		*self == MolecularEntityIri || *self == MOLECULAR_ENTITY_LABEL
	}
}
