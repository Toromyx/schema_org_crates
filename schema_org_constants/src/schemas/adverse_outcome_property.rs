/// <https://schema.org/adverseOutcome>
pub const ADVERSE_OUTCOME_PROPERTY_IRI_HTTP: &str = "http://schema.org/adverseOutcome";
/// <https://schema.org/adverseOutcome>
pub const ADVERSE_OUTCOME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/adverseOutcome";
/// <https://schema.org/adverseOutcome>
pub const ADVERSE_OUTCOME_PROPERTY_LABEL: &str = "adverseOutcome";
pub struct AdverseOutcomePropertyIri;
impl PartialEq<&str> for AdverseOutcomePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADVERSE_OUTCOME_PROPERTY_IRI_HTTP || *other == ADVERSE_OUTCOME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AdverseOutcomePropertyIri> for &str {
	fn eq(&self, other: &AdverseOutcomePropertyIri) -> bool {
		*self == ADVERSE_OUTCOME_PROPERTY_IRI_HTTP || *self == ADVERSE_OUTCOME_PROPERTY_IRI_HTTPS
	}
}
pub struct AdverseOutcomePropertyIriOrLabel;
impl PartialEq<&str> for AdverseOutcomePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdverseOutcomePropertyIri || *other == ADVERSE_OUTCOME_PROPERTY_LABEL
	}
}
impl PartialEq<AdverseOutcomePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AdverseOutcomePropertyIriOrLabel) -> bool {
		*self == AdverseOutcomePropertyIri || *self == ADVERSE_OUTCOME_PROPERTY_LABEL
	}
}
