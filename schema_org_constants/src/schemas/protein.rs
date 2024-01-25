/// <https://schema.org/Protein>
pub const PROTEIN_IRI_HTTP: &str = "http://schema.org/Protein";
/// <https://schema.org/Protein>
pub const PROTEIN_IRI_HTTPS: &str = "https://schema.org/Protein";
/// <https://schema.org/Protein>
pub const PROTEIN_LABEL: &str = "Protein";
pub struct ProteinIri;
impl PartialEq<&str> for ProteinIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROTEIN_IRI_HTTP || *other == PROTEIN_IRI_HTTPS
	}
}
impl PartialEq<ProteinIri> for &str {
	fn eq(&self, other: &ProteinIri) -> bool {
		*self == PROTEIN_IRI_HTTP || *self == PROTEIN_IRI_HTTPS
	}
}
pub struct ProteinIriOrLabel;
impl PartialEq<&str> for ProteinIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProteinIri || *other == PROTEIN_LABEL
	}
}
impl PartialEq<ProteinIriOrLabel> for &str {
	fn eq(&self, other: &ProteinIriOrLabel) -> bool {
		*self == ProteinIri || *self == PROTEIN_LABEL
	}
}
