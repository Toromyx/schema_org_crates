/// <https://schema.org/MedicalIntangible>
pub const MEDICAL_INTANGIBLE_IRI_HTTP: &str = "http://schema.org/MedicalIntangible";
/// <https://schema.org/MedicalIntangible>
pub const MEDICAL_INTANGIBLE_IRI_HTTPS: &str = "https://schema.org/MedicalIntangible";
/// <https://schema.org/MedicalIntangible>
pub const MEDICAL_INTANGIBLE_LABEL: &str = "MedicalIntangible";
pub struct MedicalIntangibleIri;
impl PartialEq<&str> for MedicalIntangibleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_INTANGIBLE_IRI_HTTP || *other == MEDICAL_INTANGIBLE_IRI_HTTPS
	}
}
impl PartialEq<MedicalIntangibleIri> for &str {
	fn eq(&self, other: &MedicalIntangibleIri) -> bool {
		*self == MEDICAL_INTANGIBLE_IRI_HTTP || *self == MEDICAL_INTANGIBLE_IRI_HTTPS
	}
}
pub struct MedicalIntangibleIriOrLabel;
impl PartialEq<&str> for MedicalIntangibleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalIntangibleIri || *other == MEDICAL_INTANGIBLE_LABEL
	}
}
impl PartialEq<MedicalIntangibleIriOrLabel> for &str {
	fn eq(&self, other: &MedicalIntangibleIriOrLabel) -> bool {
		*self == MedicalIntangibleIri || *self == MEDICAL_INTANGIBLE_LABEL
	}
}
