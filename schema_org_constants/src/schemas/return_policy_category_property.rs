/// <https://schema.org/returnPolicyCategory>
pub const RETURN_POLICY_CATEGORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/returnPolicyCategory";
/// <https://schema.org/returnPolicyCategory>
pub const RETURN_POLICY_CATEGORY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/returnPolicyCategory";
/// <https://schema.org/returnPolicyCategory>
pub const RETURN_POLICY_CATEGORY_PROPERTY_LABEL: &str = "returnPolicyCategory";
pub struct ReturnPolicyCategoryPropertyIri;
impl PartialEq<&str> for ReturnPolicyCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_POLICY_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == RETURN_POLICY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReturnPolicyCategoryPropertyIri> for &str {
	fn eq(&self, other: &ReturnPolicyCategoryPropertyIri) -> bool {
		*self == RETURN_POLICY_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == RETURN_POLICY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct ReturnPolicyCategoryPropertyIriOrLabel;
impl PartialEq<&str> for ReturnPolicyCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnPolicyCategoryPropertyIri || *other == RETURN_POLICY_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<ReturnPolicyCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReturnPolicyCategoryPropertyIriOrLabel) -> bool {
		*self == ReturnPolicyCategoryPropertyIri || *self == RETURN_POLICY_CATEGORY_PROPERTY_LABEL
	}
}
