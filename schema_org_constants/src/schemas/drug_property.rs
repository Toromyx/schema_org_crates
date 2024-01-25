/// <https://schema.org/drug>
pub const DRUG_PROPERTY_IRI_HTTP: &str = "http://schema.org/drug";
/// <https://schema.org/drug>
pub const DRUG_PROPERTY_IRI_HTTPS: &str = "https://schema.org/drug";
/// <https://schema.org/drug>
pub const DRUG_PROPERTY_LABEL: &str = "drug";
pub struct DrugPropertyIri;
impl PartialEq<&str> for DrugPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_PROPERTY_IRI_HTTP || *other == DRUG_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DrugPropertyIri> for &str {
	fn eq(&self, other: &DrugPropertyIri) -> bool {
		*self == DRUG_PROPERTY_IRI_HTTP || *self == DRUG_PROPERTY_IRI_HTTPS
	}
}
pub struct DrugPropertyIriOrLabel;
impl PartialEq<&str> for DrugPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugPropertyIri || *other == DRUG_PROPERTY_LABEL
	}
}
impl PartialEq<DrugPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DrugPropertyIriOrLabel) -> bool {
		*self == DrugPropertyIri || *self == DRUG_PROPERTY_LABEL
	}
}
