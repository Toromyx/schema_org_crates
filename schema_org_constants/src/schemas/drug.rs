/// <https://schema.org/Drug>
pub const DRUG_IRI_HTTP: &str = "http://schema.org/Drug";
/// <https://schema.org/Drug>
pub const DRUG_IRI_HTTPS: &str = "https://schema.org/Drug";
/// <https://schema.org/Drug>
pub const DRUG_LABEL: &str = "Drug";
pub struct DrugIri;
impl PartialEq<&str> for DrugIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_IRI_HTTP || *other == DRUG_IRI_HTTPS
	}
}
impl PartialEq<DrugIri> for &str {
	fn eq(&self, other: &DrugIri) -> bool {
		*self == DRUG_IRI_HTTP || *self == DRUG_IRI_HTTPS
	}
}
pub struct DrugIriOrLabel;
impl PartialEq<&str> for DrugIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugIri || *other == DRUG_LABEL
	}
}
impl PartialEq<DrugIriOrLabel> for &str {
	fn eq(&self, other: &DrugIriOrLabel) -> bool {
		*self == DrugIri || *self == DRUG_LABEL
	}
}
