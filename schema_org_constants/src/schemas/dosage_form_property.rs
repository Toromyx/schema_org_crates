/// <https://schema.org/dosageForm>
pub const DOSAGE_FORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/dosageForm";
/// <https://schema.org/dosageForm>
pub const DOSAGE_FORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dosageForm";
/// <https://schema.org/dosageForm>
pub const DOSAGE_FORM_PROPERTY_LABEL: &str = "dosageForm";
pub struct DosageFormPropertyIri;
impl PartialEq<&str> for DosageFormPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOSAGE_FORM_PROPERTY_IRI_HTTP || *other == DOSAGE_FORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DosageFormPropertyIri> for &str {
	fn eq(&self, other: &DosageFormPropertyIri) -> bool {
		*self == DOSAGE_FORM_PROPERTY_IRI_HTTP || *self == DOSAGE_FORM_PROPERTY_IRI_HTTPS
	}
}
pub struct DosageFormPropertyIriOrLabel;
impl PartialEq<&str> for DosageFormPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DosageFormPropertyIri || *other == DOSAGE_FORM_PROPERTY_LABEL
	}
}
impl PartialEq<DosageFormPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DosageFormPropertyIriOrLabel) -> bool {
		*self == DosageFormPropertyIri || *self == DOSAGE_FORM_PROPERTY_LABEL
	}
}
