/// <https://schema.org/Taxon>
pub const TAXON_IRI_HTTP: &str = "http://schema.org/Taxon";
/// <https://schema.org/Taxon>
pub const TAXON_IRI_HTTPS: &str = "https://schema.org/Taxon";
/// <https://schema.org/Taxon>
pub const TAXON_LABEL: &str = "Taxon";
pub struct TaxonIri;
impl PartialEq<&str> for TaxonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAXON_IRI_HTTP || *other == TAXON_IRI_HTTPS
	}
}
impl PartialEq<TaxonIri> for &str {
	fn eq(&self, other: &TaxonIri) -> bool {
		*self == TAXON_IRI_HTTP || *self == TAXON_IRI_HTTPS
	}
}
pub struct TaxonIriOrLabel;
impl PartialEq<&str> for TaxonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxonIri || *other == TAXON_LABEL
	}
}
impl PartialEq<TaxonIriOrLabel> for &str {
	fn eq(&self, other: &TaxonIriOrLabel) -> bool {
		*self == TaxonIri || *self == TAXON_LABEL
	}
}
