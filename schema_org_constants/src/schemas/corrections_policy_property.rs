/// <https://schema.org/correctionsPolicy>
pub const CORRECTIONS_POLICY_PROPERTY_IRI_HTTP: &str = "http://schema.org/correctionsPolicy";
/// <https://schema.org/correctionsPolicy>
pub const CORRECTIONS_POLICY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/correctionsPolicy";
/// <https://schema.org/correctionsPolicy>
pub const CORRECTIONS_POLICY_PROPERTY_LABEL: &str = "correctionsPolicy";
pub struct CorrectionsPolicyPropertyIri;
impl PartialEq<&str> for CorrectionsPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CORRECTIONS_POLICY_PROPERTY_IRI_HTTP
			|| *other == CORRECTIONS_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CorrectionsPolicyPropertyIri> for &str {
	fn eq(&self, other: &CorrectionsPolicyPropertyIri) -> bool {
		*self == CORRECTIONS_POLICY_PROPERTY_IRI_HTTP
			|| *self == CORRECTIONS_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct CorrectionsPolicyPropertyIriOrLabel;
impl PartialEq<&str> for CorrectionsPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CorrectionsPolicyPropertyIri || *other == CORRECTIONS_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<CorrectionsPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CorrectionsPolicyPropertyIriOrLabel) -> bool {
		*self == CorrectionsPolicyPropertyIri || *self == CORRECTIONS_POLICY_PROPERTY_LABEL
	}
}
