/// <https://schema.org/ReturnFeesCustomerResponsibility>
pub const RETURN_FEES_CUSTOMER_RESPONSIBILITY_IRI_HTTP: &str =
	"http://schema.org/ReturnFeesCustomerResponsibility";
/// <https://schema.org/ReturnFeesCustomerResponsibility>
pub const RETURN_FEES_CUSTOMER_RESPONSIBILITY_IRI_HTTPS: &str =
	"https://schema.org/ReturnFeesCustomerResponsibility";
/// <https://schema.org/ReturnFeesCustomerResponsibility>
pub const RETURN_FEES_CUSTOMER_RESPONSIBILITY_LABEL: &str = "ReturnFeesCustomerResponsibility";
pub struct ReturnFeesCustomerResponsibilityIri;
impl PartialEq<&str> for ReturnFeesCustomerResponsibilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_FEES_CUSTOMER_RESPONSIBILITY_IRI_HTTP
			|| *other == RETURN_FEES_CUSTOMER_RESPONSIBILITY_IRI_HTTPS
	}
}
impl PartialEq<ReturnFeesCustomerResponsibilityIri> for &str {
	fn eq(&self, other: &ReturnFeesCustomerResponsibilityIri) -> bool {
		*self == RETURN_FEES_CUSTOMER_RESPONSIBILITY_IRI_HTTP
			|| *self == RETURN_FEES_CUSTOMER_RESPONSIBILITY_IRI_HTTPS
	}
}
pub struct ReturnFeesCustomerResponsibilityIriOrLabel;
impl PartialEq<&str> for ReturnFeesCustomerResponsibilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnFeesCustomerResponsibilityIri
			|| *other == RETURN_FEES_CUSTOMER_RESPONSIBILITY_LABEL
	}
}
impl PartialEq<ReturnFeesCustomerResponsibilityIriOrLabel> for &str {
	fn eq(&self, other: &ReturnFeesCustomerResponsibilityIriOrLabel) -> bool {
		*self == ReturnFeesCustomerResponsibilityIri
			|| *self == RETURN_FEES_CUSTOMER_RESPONSIBILITY_LABEL
	}
}
