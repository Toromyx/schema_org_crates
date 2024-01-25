/// <https://schema.org/relatedDrug>
pub const RELATED_DRUG_PROPERTY_IRI_HTTP: &str = "http://schema.org/relatedDrug";
/// <https://schema.org/relatedDrug>
pub const RELATED_DRUG_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relatedDrug";
/// <https://schema.org/relatedDrug>
pub const RELATED_DRUG_PROPERTY_LABEL: &str = "relatedDrug";
pub struct RelatedDrugPropertyIri;
impl PartialEq<&str> for RelatedDrugPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELATED_DRUG_PROPERTY_IRI_HTTP || *other == RELATED_DRUG_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelatedDrugPropertyIri> for &str {
	fn eq(&self, other: &RelatedDrugPropertyIri) -> bool {
		*self == RELATED_DRUG_PROPERTY_IRI_HTTP || *self == RELATED_DRUG_PROPERTY_IRI_HTTPS
	}
}
pub struct RelatedDrugPropertyIriOrLabel;
impl PartialEq<&str> for RelatedDrugPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelatedDrugPropertyIri || *other == RELATED_DRUG_PROPERTY_LABEL
	}
}
impl PartialEq<RelatedDrugPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelatedDrugPropertyIriOrLabel) -> bool {
		*self == RelatedDrugPropertyIri || *self == RELATED_DRUG_PROPERTY_LABEL
	}
}
