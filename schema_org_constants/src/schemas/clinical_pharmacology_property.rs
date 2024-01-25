/// <https://schema.org/clinicalPharmacology>
pub const CLINICAL_PHARMACOLOGY_PROPERTY_IRI_HTTP: &str = "http://schema.org/clinicalPharmacology";
/// <https://schema.org/clinicalPharmacology>
pub const CLINICAL_PHARMACOLOGY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/clinicalPharmacology";
/// <https://schema.org/clinicalPharmacology>
pub const CLINICAL_PHARMACOLOGY_PROPERTY_LABEL: &str = "clinicalPharmacology";
pub struct ClinicalPharmacologyPropertyIri;
impl PartialEq<&str> for ClinicalPharmacologyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLINICAL_PHARMACOLOGY_PROPERTY_IRI_HTTP
			|| *other == CLINICAL_PHARMACOLOGY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ClinicalPharmacologyPropertyIri> for &str {
	fn eq(&self, other: &ClinicalPharmacologyPropertyIri) -> bool {
		*self == CLINICAL_PHARMACOLOGY_PROPERTY_IRI_HTTP
			|| *self == CLINICAL_PHARMACOLOGY_PROPERTY_IRI_HTTPS
	}
}
pub struct ClinicalPharmacologyPropertyIriOrLabel;
impl PartialEq<&str> for ClinicalPharmacologyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClinicalPharmacologyPropertyIri || *other == CLINICAL_PHARMACOLOGY_PROPERTY_LABEL
	}
}
impl PartialEq<ClinicalPharmacologyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ClinicalPharmacologyPropertyIriOrLabel) -> bool {
		*self == ClinicalPharmacologyPropertyIri || *self == CLINICAL_PHARMACOLOGY_PROPERTY_LABEL
	}
}
