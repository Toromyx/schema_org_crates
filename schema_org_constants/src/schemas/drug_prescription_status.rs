/// <https://schema.org/DrugPrescriptionStatus>
pub const DRUG_PRESCRIPTION_STATUS_IRI_HTTP: &str = "http://schema.org/DrugPrescriptionStatus";
/// <https://schema.org/DrugPrescriptionStatus>
pub const DRUG_PRESCRIPTION_STATUS_IRI_HTTPS: &str = "https://schema.org/DrugPrescriptionStatus";
/// <https://schema.org/DrugPrescriptionStatus>
pub const DRUG_PRESCRIPTION_STATUS_LABEL: &str = "DrugPrescriptionStatus";
pub struct DrugPrescriptionStatusIri;
impl PartialEq<&str> for DrugPrescriptionStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_PRESCRIPTION_STATUS_IRI_HTTP || *other == DRUG_PRESCRIPTION_STATUS_IRI_HTTPS
	}
}
impl PartialEq<DrugPrescriptionStatusIri> for &str {
	fn eq(&self, other: &DrugPrescriptionStatusIri) -> bool {
		*self == DRUG_PRESCRIPTION_STATUS_IRI_HTTP || *self == DRUG_PRESCRIPTION_STATUS_IRI_HTTPS
	}
}
pub struct DrugPrescriptionStatusIriOrLabel;
impl PartialEq<&str> for DrugPrescriptionStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugPrescriptionStatusIri || *other == DRUG_PRESCRIPTION_STATUS_LABEL
	}
}
impl PartialEq<DrugPrescriptionStatusIriOrLabel> for &str {
	fn eq(&self, other: &DrugPrescriptionStatusIriOrLabel) -> bool {
		*self == DrugPrescriptionStatusIri || *self == DRUG_PRESCRIPTION_STATUS_LABEL
	}
}
