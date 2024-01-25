/// <https://schema.org/drugUnit>
pub const DRUG_UNIT_PROPERTY_IRI_HTTP: &str = "http://schema.org/drugUnit";
/// <https://schema.org/drugUnit>
pub const DRUG_UNIT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/drugUnit";
/// <https://schema.org/drugUnit>
pub const DRUG_UNIT_PROPERTY_LABEL: &str = "drugUnit";
pub struct DrugUnitPropertyIri;
impl PartialEq<&str> for DrugUnitPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_UNIT_PROPERTY_IRI_HTTP || *other == DRUG_UNIT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DrugUnitPropertyIri> for &str {
	fn eq(&self, other: &DrugUnitPropertyIri) -> bool {
		*self == DRUG_UNIT_PROPERTY_IRI_HTTP || *self == DRUG_UNIT_PROPERTY_IRI_HTTPS
	}
}
pub struct DrugUnitPropertyIriOrLabel;
impl PartialEq<&str> for DrugUnitPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugUnitPropertyIri || *other == DRUG_UNIT_PROPERTY_LABEL
	}
}
impl PartialEq<DrugUnitPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DrugUnitPropertyIriOrLabel) -> bool {
		*self == DrugUnitPropertyIri || *self == DRUG_UNIT_PROPERTY_LABEL
	}
}
