/// <https://schema.org/actionableFeedbackPolicy>
pub const ACTIONABLE_FEEDBACK_POLICY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/actionableFeedbackPolicy";
/// <https://schema.org/actionableFeedbackPolicy>
pub const ACTIONABLE_FEEDBACK_POLICY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/actionableFeedbackPolicy";
/// <https://schema.org/actionableFeedbackPolicy>
pub const ACTIONABLE_FEEDBACK_POLICY_PROPERTY_LABEL: &str = "actionableFeedbackPolicy";
pub struct ActionableFeedbackPolicyPropertyIri;
impl PartialEq<&str> for ActionableFeedbackPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTIONABLE_FEEDBACK_POLICY_PROPERTY_IRI_HTTP
			|| *other == ACTIONABLE_FEEDBACK_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActionableFeedbackPolicyPropertyIri> for &str {
	fn eq(&self, other: &ActionableFeedbackPolicyPropertyIri) -> bool {
		*self == ACTIONABLE_FEEDBACK_POLICY_PROPERTY_IRI_HTTP
			|| *self == ACTIONABLE_FEEDBACK_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct ActionableFeedbackPolicyPropertyIriOrLabel;
impl PartialEq<&str> for ActionableFeedbackPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionableFeedbackPolicyPropertyIri
			|| *other == ACTIONABLE_FEEDBACK_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<ActionableFeedbackPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActionableFeedbackPolicyPropertyIriOrLabel) -> bool {
		*self == ActionableFeedbackPolicyPropertyIri
			|| *self == ACTIONABLE_FEEDBACK_POLICY_PROPERTY_LABEL
	}
}
