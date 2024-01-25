/// <https://schema.org/ReturnLabelCustomerResponsibility>
pub const RETURN_LABEL_CUSTOMER_RESPONSIBILITY_IRI_HTTP: &str =
	"http://schema.org/ReturnLabelCustomerResponsibility";
/// <https://schema.org/ReturnLabelCustomerResponsibility>
pub const RETURN_LABEL_CUSTOMER_RESPONSIBILITY_IRI_HTTPS: &str =
	"https://schema.org/ReturnLabelCustomerResponsibility";
/// <https://schema.org/ReturnLabelCustomerResponsibility>
pub const RETURN_LABEL_CUSTOMER_RESPONSIBILITY_LABEL: &str = "ReturnLabelCustomerResponsibility";
pub struct ReturnLabelCustomerResponsibilityIri;
impl PartialEq<&str> for ReturnLabelCustomerResponsibilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_LABEL_CUSTOMER_RESPONSIBILITY_IRI_HTTP
			|| *other == RETURN_LABEL_CUSTOMER_RESPONSIBILITY_IRI_HTTPS
	}
}
impl PartialEq<ReturnLabelCustomerResponsibilityIri> for &str {
	fn eq(&self, other: &ReturnLabelCustomerResponsibilityIri) -> bool {
		*self == RETURN_LABEL_CUSTOMER_RESPONSIBILITY_IRI_HTTP
			|| *self == RETURN_LABEL_CUSTOMER_RESPONSIBILITY_IRI_HTTPS
	}
}
pub struct ReturnLabelCustomerResponsibilityIriOrLabel;
impl PartialEq<&str> for ReturnLabelCustomerResponsibilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnLabelCustomerResponsibilityIri
			|| *other == RETURN_LABEL_CUSTOMER_RESPONSIBILITY_LABEL
	}
}
impl PartialEq<ReturnLabelCustomerResponsibilityIriOrLabel> for &str {
	fn eq(&self, other: &ReturnLabelCustomerResponsibilityIriOrLabel) -> bool {
		*self == ReturnLabelCustomerResponsibilityIri
			|| *self == RETURN_LABEL_CUSTOMER_RESPONSIBILITY_LABEL
	}
}
