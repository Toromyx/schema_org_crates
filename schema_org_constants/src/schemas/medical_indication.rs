/// <https://schema.org/MedicalIndication>
pub const MEDICAL_INDICATION_IRI_HTTP: &str = "http://schema.org/MedicalIndication";
/// <https://schema.org/MedicalIndication>
pub const MEDICAL_INDICATION_IRI_HTTPS: &str = "https://schema.org/MedicalIndication";
/// <https://schema.org/MedicalIndication>
pub const MEDICAL_INDICATION_LABEL: &str = "MedicalIndication";
pub struct MedicalIndicationIri;
impl PartialEq<&str> for MedicalIndicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_INDICATION_IRI_HTTP || *other == MEDICAL_INDICATION_IRI_HTTPS
	}
}
impl PartialEq<MedicalIndicationIri> for &str {
	fn eq(&self, other: &MedicalIndicationIri) -> bool {
		*self == MEDICAL_INDICATION_IRI_HTTP || *self == MEDICAL_INDICATION_IRI_HTTPS
	}
}
pub struct MedicalIndicationIriOrLabel;
impl PartialEq<&str> for MedicalIndicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalIndicationIri || *other == MEDICAL_INDICATION_LABEL
	}
}
impl PartialEq<MedicalIndicationIriOrLabel> for &str {
	fn eq(&self, other: &MedicalIndicationIriOrLabel) -> bool {
		*self == MedicalIndicationIri || *self == MEDICAL_INDICATION_LABEL
	}
}
