/// <https://schema.org/noBylinesPolicy>
pub const NO_BYLINES_POLICY_PROPERTY_IRI_HTTP: &str = "http://schema.org/noBylinesPolicy";
/// <https://schema.org/noBylinesPolicy>
pub const NO_BYLINES_POLICY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/noBylinesPolicy";
/// <https://schema.org/noBylinesPolicy>
pub const NO_BYLINES_POLICY_PROPERTY_LABEL: &str = "noBylinesPolicy";
pub struct NoBylinesPolicyPropertyIri;
impl PartialEq<&str> for NoBylinesPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NO_BYLINES_POLICY_PROPERTY_IRI_HTTP
			|| *other == NO_BYLINES_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NoBylinesPolicyPropertyIri> for &str {
	fn eq(&self, other: &NoBylinesPolicyPropertyIri) -> bool {
		*self == NO_BYLINES_POLICY_PROPERTY_IRI_HTTP
			|| *self == NO_BYLINES_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct NoBylinesPolicyPropertyIriOrLabel;
impl PartialEq<&str> for NoBylinesPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NoBylinesPolicyPropertyIri || *other == NO_BYLINES_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<NoBylinesPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NoBylinesPolicyPropertyIriOrLabel) -> bool {
		*self == NoBylinesPolicyPropertyIri || *self == NO_BYLINES_POLICY_PROPERTY_LABEL
	}
}
