/// <https://schema.org/TreatmentIndication>
pub const TREATMENT_INDICATION_IRI_HTTP: &str = "http://schema.org/TreatmentIndication";
/// <https://schema.org/TreatmentIndication>
pub const TREATMENT_INDICATION_IRI_HTTPS: &str = "https://schema.org/TreatmentIndication";
/// <https://schema.org/TreatmentIndication>
pub const TREATMENT_INDICATION_LABEL: &str = "TreatmentIndication";
pub struct TreatmentIndicationIri;
impl PartialEq<&str> for TreatmentIndicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TREATMENT_INDICATION_IRI_HTTP || *other == TREATMENT_INDICATION_IRI_HTTPS
	}
}
impl PartialEq<TreatmentIndicationIri> for &str {
	fn eq(&self, other: &TreatmentIndicationIri) -> bool {
		*self == TREATMENT_INDICATION_IRI_HTTP || *self == TREATMENT_INDICATION_IRI_HTTPS
	}
}
pub struct TreatmentIndicationIriOrLabel;
impl PartialEq<&str> for TreatmentIndicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TreatmentIndicationIri || *other == TREATMENT_INDICATION_LABEL
	}
}
impl PartialEq<TreatmentIndicationIriOrLabel> for &str {
	fn eq(&self, other: &TreatmentIndicationIriOrLabel) -> bool {
		*self == TreatmentIndicationIri || *self == TREATMENT_INDICATION_LABEL
	}
}
