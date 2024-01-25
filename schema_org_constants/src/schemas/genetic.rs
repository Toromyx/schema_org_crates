/// <https://schema.org/Genetic>
pub const GENETIC_IRI_HTTP: &str = "http://schema.org/Genetic";
/// <https://schema.org/Genetic>
pub const GENETIC_IRI_HTTPS: &str = "https://schema.org/Genetic";
/// <https://schema.org/Genetic>
pub const GENETIC_LABEL: &str = "Genetic";
pub struct GeneticIri;
impl PartialEq<&str> for GeneticIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GENETIC_IRI_HTTP || *other == GENETIC_IRI_HTTPS
	}
}
impl PartialEq<GeneticIri> for &str {
	fn eq(&self, other: &GeneticIri) -> bool {
		*self == GENETIC_IRI_HTTP || *self == GENETIC_IRI_HTTPS
	}
}
pub struct GeneticIriOrLabel;
impl PartialEq<&str> for GeneticIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeneticIri || *other == GENETIC_LABEL
	}
}
impl PartialEq<GeneticIriOrLabel> for &str {
	fn eq(&self, other: &GeneticIriOrLabel) -> bool {
		*self == GeneticIri || *self == GENETIC_LABEL
	}
}
