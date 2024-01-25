/// <https://schema.org/DrugLegalStatus>
pub const DRUG_LEGAL_STATUS_IRI_HTTP: &str = "http://schema.org/DrugLegalStatus";
/// <https://schema.org/DrugLegalStatus>
pub const DRUG_LEGAL_STATUS_IRI_HTTPS: &str = "https://schema.org/DrugLegalStatus";
/// <https://schema.org/DrugLegalStatus>
pub const DRUG_LEGAL_STATUS_LABEL: &str = "DrugLegalStatus";
pub struct DrugLegalStatusIri;
impl PartialEq<&str> for DrugLegalStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_LEGAL_STATUS_IRI_HTTP || *other == DRUG_LEGAL_STATUS_IRI_HTTPS
	}
}
impl PartialEq<DrugLegalStatusIri> for &str {
	fn eq(&self, other: &DrugLegalStatusIri) -> bool {
		*self == DRUG_LEGAL_STATUS_IRI_HTTP || *self == DRUG_LEGAL_STATUS_IRI_HTTPS
	}
}
pub struct DrugLegalStatusIriOrLabel;
impl PartialEq<&str> for DrugLegalStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugLegalStatusIri || *other == DRUG_LEGAL_STATUS_LABEL
	}
}
impl PartialEq<DrugLegalStatusIriOrLabel> for &str {
	fn eq(&self, other: &DrugLegalStatusIriOrLabel) -> bool {
		*self == DrugLegalStatusIri || *self == DRUG_LEGAL_STATUS_LABEL
	}
}
