/// <https://schema.org/businessFunction>
pub const BUSINESS_FUNCTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/businessFunction";
/// <https://schema.org/businessFunction>
pub const BUSINESS_FUNCTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/businessFunction";
/// <https://schema.org/businessFunction>
pub const BUSINESS_FUNCTION_PROPERTY_LABEL: &str = "businessFunction";
pub struct BusinessFunctionPropertyIri;
impl PartialEq<&str> for BusinessFunctionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUSINESS_FUNCTION_PROPERTY_IRI_HTTP
			|| *other == BUSINESS_FUNCTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BusinessFunctionPropertyIri> for &str {
	fn eq(&self, other: &BusinessFunctionPropertyIri) -> bool {
		*self == BUSINESS_FUNCTION_PROPERTY_IRI_HTTP
			|| *self == BUSINESS_FUNCTION_PROPERTY_IRI_HTTPS
	}
}
pub struct BusinessFunctionPropertyIriOrLabel;
impl PartialEq<&str> for BusinessFunctionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusinessFunctionPropertyIri || *other == BUSINESS_FUNCTION_PROPERTY_LABEL
	}
}
impl PartialEq<BusinessFunctionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BusinessFunctionPropertyIriOrLabel) -> bool {
		*self == BusinessFunctionPropertyIri || *self == BUSINESS_FUNCTION_PROPERTY_LABEL
	}
}
