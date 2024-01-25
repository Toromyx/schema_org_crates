/// <https://schema.org/drugClass>
pub const DRUG_CLASS_PROPERTY_IRI_HTTP: &str = "http://schema.org/drugClass";
/// <https://schema.org/drugClass>
pub const DRUG_CLASS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/drugClass";
/// <https://schema.org/drugClass>
pub const DRUG_CLASS_PROPERTY_LABEL: &str = "drugClass";
pub struct DrugClassPropertyIri;
impl PartialEq<&str> for DrugClassPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_CLASS_PROPERTY_IRI_HTTP || *other == DRUG_CLASS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DrugClassPropertyIri> for &str {
	fn eq(&self, other: &DrugClassPropertyIri) -> bool {
		*self == DRUG_CLASS_PROPERTY_IRI_HTTP || *self == DRUG_CLASS_PROPERTY_IRI_HTTPS
	}
}
pub struct DrugClassPropertyIriOrLabel;
impl PartialEq<&str> for DrugClassPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugClassPropertyIri || *other == DRUG_CLASS_PROPERTY_LABEL
	}
}
impl PartialEq<DrugClassPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DrugClassPropertyIriOrLabel) -> bool {
		*self == DrugClassPropertyIri || *self == DRUG_CLASS_PROPERTY_LABEL
	}
}
