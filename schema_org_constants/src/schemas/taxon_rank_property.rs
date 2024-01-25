/// <https://schema.org/taxonRank>
pub const TAXON_RANK_PROPERTY_IRI_HTTP: &str = "http://schema.org/taxonRank";
/// <https://schema.org/taxonRank>
pub const TAXON_RANK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/taxonRank";
/// <https://schema.org/taxonRank>
pub const TAXON_RANK_PROPERTY_LABEL: &str = "taxonRank";
pub struct TaxonRankPropertyIri;
impl PartialEq<&str> for TaxonRankPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAXON_RANK_PROPERTY_IRI_HTTP || *other == TAXON_RANK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TaxonRankPropertyIri> for &str {
	fn eq(&self, other: &TaxonRankPropertyIri) -> bool {
		*self == TAXON_RANK_PROPERTY_IRI_HTTP || *self == TAXON_RANK_PROPERTY_IRI_HTTPS
	}
}
pub struct TaxonRankPropertyIriOrLabel;
impl PartialEq<&str> for TaxonRankPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxonRankPropertyIri || *other == TAXON_RANK_PROPERTY_LABEL
	}
}
impl PartialEq<TaxonRankPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TaxonRankPropertyIriOrLabel) -> bool {
		*self == TaxonRankPropertyIri || *self == TAXON_RANK_PROPERTY_LABEL
	}
}
