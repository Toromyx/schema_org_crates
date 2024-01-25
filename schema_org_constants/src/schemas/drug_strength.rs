/// <https://schema.org/DrugStrength>
pub const DRUG_STRENGTH_IRI_HTTP: &str = "http://schema.org/DrugStrength";
/// <https://schema.org/DrugStrength>
pub const DRUG_STRENGTH_IRI_HTTPS: &str = "https://schema.org/DrugStrength";
/// <https://schema.org/DrugStrength>
pub const DRUG_STRENGTH_LABEL: &str = "DrugStrength";
pub struct DrugStrengthIri;
impl PartialEq<&str> for DrugStrengthIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_STRENGTH_IRI_HTTP || *other == DRUG_STRENGTH_IRI_HTTPS
	}
}
impl PartialEq<DrugStrengthIri> for &str {
	fn eq(&self, other: &DrugStrengthIri) -> bool {
		*self == DRUG_STRENGTH_IRI_HTTP || *self == DRUG_STRENGTH_IRI_HTTPS
	}
}
pub struct DrugStrengthIriOrLabel;
impl PartialEq<&str> for DrugStrengthIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugStrengthIri || *other == DRUG_STRENGTH_LABEL
	}
}
impl PartialEq<DrugStrengthIriOrLabel> for &str {
	fn eq(&self, other: &DrugStrengthIriOrLabel) -> bool {
		*self == DrugStrengthIri || *self == DRUG_STRENGTH_LABEL
	}
}
