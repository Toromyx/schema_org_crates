/// <https://schema.org/DrugPregnancyCategory>
pub const DRUG_PREGNANCY_CATEGORY_IRI_HTTP: &str = "http://schema.org/DrugPregnancyCategory";
/// <https://schema.org/DrugPregnancyCategory>
pub const DRUG_PREGNANCY_CATEGORY_IRI_HTTPS: &str = "https://schema.org/DrugPregnancyCategory";
/// <https://schema.org/DrugPregnancyCategory>
pub const DRUG_PREGNANCY_CATEGORY_LABEL: &str = "DrugPregnancyCategory";
pub struct DrugPregnancyCategoryIri;
impl PartialEq<&str> for DrugPregnancyCategoryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_PREGNANCY_CATEGORY_IRI_HTTP || *other == DRUG_PREGNANCY_CATEGORY_IRI_HTTPS
	}
}
impl PartialEq<DrugPregnancyCategoryIri> for &str {
	fn eq(&self, other: &DrugPregnancyCategoryIri) -> bool {
		*self == DRUG_PREGNANCY_CATEGORY_IRI_HTTP || *self == DRUG_PREGNANCY_CATEGORY_IRI_HTTPS
	}
}
pub struct DrugPregnancyCategoryIriOrLabel;
impl PartialEq<&str> for DrugPregnancyCategoryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugPregnancyCategoryIri || *other == DRUG_PREGNANCY_CATEGORY_LABEL
	}
}
impl PartialEq<DrugPregnancyCategoryIriOrLabel> for &str {
	fn eq(&self, other: &DrugPregnancyCategoryIriOrLabel) -> bool {
		*self == DrugPregnancyCategoryIri || *self == DRUG_PREGNANCY_CATEGORY_LABEL
	}
}
