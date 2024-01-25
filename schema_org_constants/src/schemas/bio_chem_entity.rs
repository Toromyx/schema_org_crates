/// <https://schema.org/BioChemEntity>
pub const BIO_CHEM_ENTITY_IRI_HTTP: &str = "http://schema.org/BioChemEntity";
/// <https://schema.org/BioChemEntity>
pub const BIO_CHEM_ENTITY_IRI_HTTPS: &str = "https://schema.org/BioChemEntity";
/// <https://schema.org/BioChemEntity>
pub const BIO_CHEM_ENTITY_LABEL: &str = "BioChemEntity";
pub struct BioChemEntityIri;
impl PartialEq<&str> for BioChemEntityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BIO_CHEM_ENTITY_IRI_HTTP || *other == BIO_CHEM_ENTITY_IRI_HTTPS
	}
}
impl PartialEq<BioChemEntityIri> for &str {
	fn eq(&self, other: &BioChemEntityIri) -> bool {
		*self == BIO_CHEM_ENTITY_IRI_HTTP || *self == BIO_CHEM_ENTITY_IRI_HTTPS
	}
}
pub struct BioChemEntityIriOrLabel;
impl PartialEq<&str> for BioChemEntityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BioChemEntityIri || *other == BIO_CHEM_ENTITY_LABEL
	}
}
impl PartialEq<BioChemEntityIriOrLabel> for &str {
	fn eq(&self, other: &BioChemEntityIriOrLabel) -> bool {
		*self == BioChemEntityIri || *self == BIO_CHEM_ENTITY_LABEL
	}
}
