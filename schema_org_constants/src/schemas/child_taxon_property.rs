/// <https://schema.org/childTaxon>
pub const CHILD_TAXON_PROPERTY_IRI_HTTP: &str = "http://schema.org/childTaxon";
/// <https://schema.org/childTaxon>
pub const CHILD_TAXON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/childTaxon";
/// <https://schema.org/childTaxon>
pub const CHILD_TAXON_PROPERTY_LABEL: &str = "childTaxon";
pub struct ChildTaxonPropertyIri;
impl PartialEq<&str> for ChildTaxonPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHILD_TAXON_PROPERTY_IRI_HTTP || *other == CHILD_TAXON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ChildTaxonPropertyIri> for &str {
	fn eq(&self, other: &ChildTaxonPropertyIri) -> bool {
		*self == CHILD_TAXON_PROPERTY_IRI_HTTP || *self == CHILD_TAXON_PROPERTY_IRI_HTTPS
	}
}
pub struct ChildTaxonPropertyIriOrLabel;
impl PartialEq<&str> for ChildTaxonPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChildTaxonPropertyIri || *other == CHILD_TAXON_PROPERTY_LABEL
	}
}
impl PartialEq<ChildTaxonPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ChildTaxonPropertyIriOrLabel) -> bool {
		*self == ChildTaxonPropertyIri || *self == CHILD_TAXON_PROPERTY_LABEL
	}
}
