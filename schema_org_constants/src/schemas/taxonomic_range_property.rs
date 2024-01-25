/// <https://schema.org/taxonomicRange>
pub const TAXONOMIC_RANGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/taxonomicRange";
/// <https://schema.org/taxonomicRange>
pub const TAXONOMIC_RANGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/taxonomicRange";
/// <https://schema.org/taxonomicRange>
pub const TAXONOMIC_RANGE_PROPERTY_LABEL: &str = "taxonomicRange";
pub struct TaxonomicRangePropertyIri;
impl PartialEq<&str> for TaxonomicRangePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAXONOMIC_RANGE_PROPERTY_IRI_HTTP || *other == TAXONOMIC_RANGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TaxonomicRangePropertyIri> for &str {
	fn eq(&self, other: &TaxonomicRangePropertyIri) -> bool {
		*self == TAXONOMIC_RANGE_PROPERTY_IRI_HTTP || *self == TAXONOMIC_RANGE_PROPERTY_IRI_HTTPS
	}
}
pub struct TaxonomicRangePropertyIriOrLabel;
impl PartialEq<&str> for TaxonomicRangePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxonomicRangePropertyIri || *other == TAXONOMIC_RANGE_PROPERTY_LABEL
	}
}
impl PartialEq<TaxonomicRangePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TaxonomicRangePropertyIriOrLabel) -> bool {
		*self == TaxonomicRangePropertyIri || *self == TAXONOMIC_RANGE_PROPERTY_LABEL
	}
}
