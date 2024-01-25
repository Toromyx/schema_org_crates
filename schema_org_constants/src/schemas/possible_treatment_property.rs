/// <https://schema.org/possibleTreatment>
pub const POSSIBLE_TREATMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/possibleTreatment";
/// <https://schema.org/possibleTreatment>
pub const POSSIBLE_TREATMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/possibleTreatment";
/// <https://schema.org/possibleTreatment>
pub const POSSIBLE_TREATMENT_PROPERTY_LABEL: &str = "possibleTreatment";
pub struct PossibleTreatmentPropertyIri;
impl PartialEq<&str> for PossibleTreatmentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSSIBLE_TREATMENT_PROPERTY_IRI_HTTP
			|| *other == POSSIBLE_TREATMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PossibleTreatmentPropertyIri> for &str {
	fn eq(&self, other: &PossibleTreatmentPropertyIri) -> bool {
		*self == POSSIBLE_TREATMENT_PROPERTY_IRI_HTTP
			|| *self == POSSIBLE_TREATMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct PossibleTreatmentPropertyIriOrLabel;
impl PartialEq<&str> for PossibleTreatmentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PossibleTreatmentPropertyIri || *other == POSSIBLE_TREATMENT_PROPERTY_LABEL
	}
}
impl PartialEq<PossibleTreatmentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PossibleTreatmentPropertyIriOrLabel) -> bool {
		*self == PossibleTreatmentPropertyIri || *self == POSSIBLE_TREATMENT_PROPERTY_LABEL
	}
}
