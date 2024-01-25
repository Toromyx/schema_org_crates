/// <https://schema.org/interactingDrug>
pub const INTERACTING_DRUG_PROPERTY_IRI_HTTP: &str = "http://schema.org/interactingDrug";
/// <https://schema.org/interactingDrug>
pub const INTERACTING_DRUG_PROPERTY_IRI_HTTPS: &str = "https://schema.org/interactingDrug";
/// <https://schema.org/interactingDrug>
pub const INTERACTING_DRUG_PROPERTY_LABEL: &str = "interactingDrug";
pub struct InteractingDrugPropertyIri;
impl PartialEq<&str> for InteractingDrugPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERACTING_DRUG_PROPERTY_IRI_HTTP
			|| *other == INTERACTING_DRUG_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InteractingDrugPropertyIri> for &str {
	fn eq(&self, other: &InteractingDrugPropertyIri) -> bool {
		*self == INTERACTING_DRUG_PROPERTY_IRI_HTTP || *self == INTERACTING_DRUG_PROPERTY_IRI_HTTPS
	}
}
pub struct InteractingDrugPropertyIriOrLabel;
impl PartialEq<&str> for InteractingDrugPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InteractingDrugPropertyIri || *other == INTERACTING_DRUG_PROPERTY_LABEL
	}
}
impl PartialEq<InteractingDrugPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InteractingDrugPropertyIriOrLabel) -> bool {
		*self == InteractingDrugPropertyIri || *self == INTERACTING_DRUG_PROPERTY_LABEL
	}
}
