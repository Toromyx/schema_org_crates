/// <https://schema.org/Gene>
pub const GENE_IRI_HTTP: &str = "http://schema.org/Gene";
/// <https://schema.org/Gene>
pub const GENE_IRI_HTTPS: &str = "https://schema.org/Gene";
/// <https://schema.org/Gene>
pub const GENE_LABEL: &str = "Gene";
pub struct GeneIri;
impl PartialEq<&str> for GeneIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GENE_IRI_HTTP || *other == GENE_IRI_HTTPS
	}
}
impl PartialEq<GeneIri> for &str {
	fn eq(&self, other: &GeneIri) -> bool {
		*self == GENE_IRI_HTTP || *self == GENE_IRI_HTTPS
	}
}
pub struct GeneIriOrLabel;
impl PartialEq<&str> for GeneIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeneIri || *other == GENE_LABEL
	}
}
impl PartialEq<GeneIriOrLabel> for &str {
	fn eq(&self, other: &GeneIriOrLabel) -> bool {
		*self == GeneIri || *self == GENE_LABEL
	}
}
