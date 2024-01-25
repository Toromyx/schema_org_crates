/// <https://schema.org/BusinessFunction>
pub const BUSINESS_FUNCTION_IRI_HTTP: &str = "http://schema.org/BusinessFunction";
/// <https://schema.org/BusinessFunction>
pub const BUSINESS_FUNCTION_IRI_HTTPS: &str = "https://schema.org/BusinessFunction";
/// <https://schema.org/BusinessFunction>
pub const BUSINESS_FUNCTION_LABEL: &str = "BusinessFunction";
pub struct BusinessFunctionIri;
impl PartialEq<&str> for BusinessFunctionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUSINESS_FUNCTION_IRI_HTTP || *other == BUSINESS_FUNCTION_IRI_HTTPS
	}
}
impl PartialEq<BusinessFunctionIri> for &str {
	fn eq(&self, other: &BusinessFunctionIri) -> bool {
		*self == BUSINESS_FUNCTION_IRI_HTTP || *self == BUSINESS_FUNCTION_IRI_HTTPS
	}
}
pub struct BusinessFunctionIriOrLabel;
impl PartialEq<&str> for BusinessFunctionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusinessFunctionIri || *other == BUSINESS_FUNCTION_LABEL
	}
}
impl PartialEq<BusinessFunctionIriOrLabel> for &str {
	fn eq(&self, other: &BusinessFunctionIriOrLabel) -> bool {
		*self == BusinessFunctionIri || *self == BUSINESS_FUNCTION_LABEL
	}
}
