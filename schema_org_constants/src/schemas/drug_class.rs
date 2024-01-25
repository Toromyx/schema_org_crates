/// <https://schema.org/DrugClass>
pub const DRUG_CLASS_IRI_HTTP: &str = "http://schema.org/DrugClass";
/// <https://schema.org/DrugClass>
pub const DRUG_CLASS_IRI_HTTPS: &str = "https://schema.org/DrugClass";
/// <https://schema.org/DrugClass>
pub const DRUG_CLASS_LABEL: &str = "DrugClass";
pub struct DrugClassIri;
impl PartialEq<&str> for DrugClassIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_CLASS_IRI_HTTP || *other == DRUG_CLASS_IRI_HTTPS
	}
}
impl PartialEq<DrugClassIri> for &str {
	fn eq(&self, other: &DrugClassIri) -> bool {
		*self == DRUG_CLASS_IRI_HTTP || *self == DRUG_CLASS_IRI_HTTPS
	}
}
pub struct DrugClassIriOrLabel;
impl PartialEq<&str> for DrugClassIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugClassIri || *other == DRUG_CLASS_LABEL
	}
}
impl PartialEq<DrugClassIriOrLabel> for &str {
	fn eq(&self, other: &DrugClassIriOrLabel) -> bool {
		*self == DrugClassIri || *self == DRUG_CLASS_LABEL
	}
}
