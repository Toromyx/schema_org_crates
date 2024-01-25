/// <https://schema.org/bioChemInteraction>
pub const BIO_CHEM_INTERACTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/bioChemInteraction";
/// <https://schema.org/bioChemInteraction>
pub const BIO_CHEM_INTERACTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bioChemInteraction";
/// <https://schema.org/bioChemInteraction>
pub const BIO_CHEM_INTERACTION_PROPERTY_LABEL: &str = "bioChemInteraction";
pub struct BioChemInteractionPropertyIri;
impl PartialEq<&str> for BioChemInteractionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BIO_CHEM_INTERACTION_PROPERTY_IRI_HTTP
			|| *other == BIO_CHEM_INTERACTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BioChemInteractionPropertyIri> for &str {
	fn eq(&self, other: &BioChemInteractionPropertyIri) -> bool {
		*self == BIO_CHEM_INTERACTION_PROPERTY_IRI_HTTP
			|| *self == BIO_CHEM_INTERACTION_PROPERTY_IRI_HTTPS
	}
}
pub struct BioChemInteractionPropertyIriOrLabel;
impl PartialEq<&str> for BioChemInteractionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BioChemInteractionPropertyIri || *other == BIO_CHEM_INTERACTION_PROPERTY_LABEL
	}
}
impl PartialEq<BioChemInteractionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BioChemInteractionPropertyIriOrLabel) -> bool {
		*self == BioChemInteractionPropertyIri || *self == BIO_CHEM_INTERACTION_PROPERTY_LABEL
	}
}
