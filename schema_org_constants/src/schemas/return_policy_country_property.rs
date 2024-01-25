/// <https://schema.org/returnPolicyCountry>
pub const RETURN_POLICY_COUNTRY_PROPERTY_IRI_HTTP: &str = "http://schema.org/returnPolicyCountry";
/// <https://schema.org/returnPolicyCountry>
pub const RETURN_POLICY_COUNTRY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/returnPolicyCountry";
/// <https://schema.org/returnPolicyCountry>
pub const RETURN_POLICY_COUNTRY_PROPERTY_LABEL: &str = "returnPolicyCountry";
pub struct ReturnPolicyCountryPropertyIri;
impl PartialEq<&str> for ReturnPolicyCountryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_POLICY_COUNTRY_PROPERTY_IRI_HTTP
			|| *other == RETURN_POLICY_COUNTRY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReturnPolicyCountryPropertyIri> for &str {
	fn eq(&self, other: &ReturnPolicyCountryPropertyIri) -> bool {
		*self == RETURN_POLICY_COUNTRY_PROPERTY_IRI_HTTP
			|| *self == RETURN_POLICY_COUNTRY_PROPERTY_IRI_HTTPS
	}
}
pub struct ReturnPolicyCountryPropertyIriOrLabel;
impl PartialEq<&str> for ReturnPolicyCountryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnPolicyCountryPropertyIri || *other == RETURN_POLICY_COUNTRY_PROPERTY_LABEL
	}
}
impl PartialEq<ReturnPolicyCountryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReturnPolicyCountryPropertyIriOrLabel) -> bool {
		*self == ReturnPolicyCountryPropertyIri || *self == RETURN_POLICY_COUNTRY_PROPERTY_LABEL
	}
}
