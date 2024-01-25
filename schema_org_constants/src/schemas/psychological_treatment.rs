/// <https://schema.org/PsychologicalTreatment>
pub const PSYCHOLOGICAL_TREATMENT_IRI_HTTP: &str = "http://schema.org/PsychologicalTreatment";
/// <https://schema.org/PsychologicalTreatment>
pub const PSYCHOLOGICAL_TREATMENT_IRI_HTTPS: &str = "https://schema.org/PsychologicalTreatment";
/// <https://schema.org/PsychologicalTreatment>
pub const PSYCHOLOGICAL_TREATMENT_LABEL: &str = "PsychologicalTreatment";
pub struct PsychologicalTreatmentIri;
impl PartialEq<&str> for PsychologicalTreatmentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PSYCHOLOGICAL_TREATMENT_IRI_HTTP || *other == PSYCHOLOGICAL_TREATMENT_IRI_HTTPS
	}
}
impl PartialEq<PsychologicalTreatmentIri> for &str {
	fn eq(&self, other: &PsychologicalTreatmentIri) -> bool {
		*self == PSYCHOLOGICAL_TREATMENT_IRI_HTTP || *self == PSYCHOLOGICAL_TREATMENT_IRI_HTTPS
	}
}
pub struct PsychologicalTreatmentIriOrLabel;
impl PartialEq<&str> for PsychologicalTreatmentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PsychologicalTreatmentIri || *other == PSYCHOLOGICAL_TREATMENT_LABEL
	}
}
impl PartialEq<PsychologicalTreatmentIriOrLabel> for &str {
	fn eq(&self, other: &PsychologicalTreatmentIriOrLabel) -> bool {
		*self == PsychologicalTreatmentIri || *self == PSYCHOLOGICAL_TREATMENT_LABEL
	}
}
