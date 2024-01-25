/// <https://schema.org/seriousAdverseOutcome>
pub const SERIOUS_ADVERSE_OUTCOME_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/seriousAdverseOutcome";
/// <https://schema.org/seriousAdverseOutcome>
pub const SERIOUS_ADVERSE_OUTCOME_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/seriousAdverseOutcome";
/// <https://schema.org/seriousAdverseOutcome>
pub const SERIOUS_ADVERSE_OUTCOME_PROPERTY_LABEL: &str = "seriousAdverseOutcome";
pub struct SeriousAdverseOutcomePropertyIri;
impl PartialEq<&str> for SeriousAdverseOutcomePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERIOUS_ADVERSE_OUTCOME_PROPERTY_IRI_HTTP
			|| *other == SERIOUS_ADVERSE_OUTCOME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeriousAdverseOutcomePropertyIri> for &str {
	fn eq(&self, other: &SeriousAdverseOutcomePropertyIri) -> bool {
		*self == SERIOUS_ADVERSE_OUTCOME_PROPERTY_IRI_HTTP
			|| *self == SERIOUS_ADVERSE_OUTCOME_PROPERTY_IRI_HTTPS
	}
}
pub struct SeriousAdverseOutcomePropertyIriOrLabel;
impl PartialEq<&str> for SeriousAdverseOutcomePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeriousAdverseOutcomePropertyIri
			|| *other == SERIOUS_ADVERSE_OUTCOME_PROPERTY_LABEL
	}
}
impl PartialEq<SeriousAdverseOutcomePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeriousAdverseOutcomePropertyIriOrLabel) -> bool {
		*self == SeriousAdverseOutcomePropertyIri || *self == SERIOUS_ADVERSE_OUTCOME_PROPERTY_LABEL
	}
}
