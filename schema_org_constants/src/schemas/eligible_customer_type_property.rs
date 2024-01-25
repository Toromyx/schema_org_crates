/// <https://schema.org/eligibleCustomerType>
pub const ELIGIBLE_CUSTOMER_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/eligibleCustomerType";
/// <https://schema.org/eligibleCustomerType>
pub const ELIGIBLE_CUSTOMER_TYPE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/eligibleCustomerType";
/// <https://schema.org/eligibleCustomerType>
pub const ELIGIBLE_CUSTOMER_TYPE_PROPERTY_LABEL: &str = "eligibleCustomerType";
pub struct EligibleCustomerTypePropertyIri;
impl PartialEq<&str> for EligibleCustomerTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELIGIBLE_CUSTOMER_TYPE_PROPERTY_IRI_HTTP
			|| *other == ELIGIBLE_CUSTOMER_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EligibleCustomerTypePropertyIri> for &str {
	fn eq(&self, other: &EligibleCustomerTypePropertyIri) -> bool {
		*self == ELIGIBLE_CUSTOMER_TYPE_PROPERTY_IRI_HTTP
			|| *self == ELIGIBLE_CUSTOMER_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct EligibleCustomerTypePropertyIriOrLabel;
impl PartialEq<&str> for EligibleCustomerTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EligibleCustomerTypePropertyIri || *other == ELIGIBLE_CUSTOMER_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<EligibleCustomerTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EligibleCustomerTypePropertyIriOrLabel) -> bool {
		*self == EligibleCustomerTypePropertyIri || *self == ELIGIBLE_CUSTOMER_TYPE_PROPERTY_LABEL
	}
}
