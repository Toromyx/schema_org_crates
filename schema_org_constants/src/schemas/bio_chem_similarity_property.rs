/// <https://schema.org/bioChemSimilarity>
pub const BIO_CHEM_SIMILARITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/bioChemSimilarity";
/// <https://schema.org/bioChemSimilarity>
pub const BIO_CHEM_SIMILARITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bioChemSimilarity";
/// <https://schema.org/bioChemSimilarity>
pub const BIO_CHEM_SIMILARITY_PROPERTY_LABEL: &str = "bioChemSimilarity";
pub struct BioChemSimilarityPropertyIri;
impl PartialEq<&str> for BioChemSimilarityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BIO_CHEM_SIMILARITY_PROPERTY_IRI_HTTP
			|| *other == BIO_CHEM_SIMILARITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BioChemSimilarityPropertyIri> for &str {
	fn eq(&self, other: &BioChemSimilarityPropertyIri) -> bool {
		*self == BIO_CHEM_SIMILARITY_PROPERTY_IRI_HTTP
			|| *self == BIO_CHEM_SIMILARITY_PROPERTY_IRI_HTTPS
	}
}
pub struct BioChemSimilarityPropertyIriOrLabel;
impl PartialEq<&str> for BioChemSimilarityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BioChemSimilarityPropertyIri || *other == BIO_CHEM_SIMILARITY_PROPERTY_LABEL
	}
}
impl PartialEq<BioChemSimilarityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BioChemSimilarityPropertyIriOrLabel) -> bool {
		*self == BioChemSimilarityPropertyIri || *self == BIO_CHEM_SIMILARITY_PROPERTY_LABEL
	}
}
