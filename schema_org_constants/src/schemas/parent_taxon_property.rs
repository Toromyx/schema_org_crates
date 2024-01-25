/// <https://schema.org/parentTaxon>
pub const PARENT_TAXON_PROPERTY_IRI_HTTP: &str = "http://schema.org/parentTaxon";
/// <https://schema.org/parentTaxon>
pub const PARENT_TAXON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/parentTaxon";
/// <https://schema.org/parentTaxon>
pub const PARENT_TAXON_PROPERTY_LABEL: &str = "parentTaxon";
pub struct ParentTaxonPropertyIri;
impl PartialEq<&str> for ParentTaxonPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARENT_TAXON_PROPERTY_IRI_HTTP || *other == PARENT_TAXON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ParentTaxonPropertyIri> for &str {
	fn eq(&self, other: &ParentTaxonPropertyIri) -> bool {
		*self == PARENT_TAXON_PROPERTY_IRI_HTTP || *self == PARENT_TAXON_PROPERTY_IRI_HTTPS
	}
}
pub struct ParentTaxonPropertyIriOrLabel;
impl PartialEq<&str> for ParentTaxonPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParentTaxonPropertyIri || *other == PARENT_TAXON_PROPERTY_LABEL
	}
}
impl PartialEq<ParentTaxonPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ParentTaxonPropertyIriOrLabel) -> bool {
		*self == ParentTaxonPropertyIri || *self == PARENT_TAXON_PROPERTY_LABEL
	}
}
