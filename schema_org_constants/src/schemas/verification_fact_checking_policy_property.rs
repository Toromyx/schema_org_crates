/// <https://schema.org/verificationFactCheckingPolicy>
pub const VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/verificationFactCheckingPolicy";
/// <https://schema.org/verificationFactCheckingPolicy>
pub const VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/verificationFactCheckingPolicy";
/// <https://schema.org/verificationFactCheckingPolicy>
pub const VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_LABEL: &str = "verificationFactCheckingPolicy";
pub struct VerificationFactCheckingPolicyPropertyIri;
impl PartialEq<&str> for VerificationFactCheckingPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_IRI_HTTP
			|| *other == VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VerificationFactCheckingPolicyPropertyIri> for &str {
	fn eq(&self, other: &VerificationFactCheckingPolicyPropertyIri) -> bool {
		*self == VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_IRI_HTTP
			|| *self == VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct VerificationFactCheckingPolicyPropertyIriOrLabel;
impl PartialEq<&str> for VerificationFactCheckingPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VerificationFactCheckingPolicyPropertyIri
			|| *other == VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<VerificationFactCheckingPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VerificationFactCheckingPolicyPropertyIriOrLabel) -> bool {
		*self == VerificationFactCheckingPolicyPropertyIri
			|| *self == VERIFICATION_FACT_CHECKING_POLICY_PROPERTY_LABEL
	}
}
