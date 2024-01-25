/// <https://schema.org/customerRemorseReturnFees>
pub const CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/customerRemorseReturnFees";
/// <https://schema.org/customerRemorseReturnFees>
pub const CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/customerRemorseReturnFees";
/// <https://schema.org/customerRemorseReturnFees>
pub const CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_LABEL: &str = "customerRemorseReturnFees";
pub struct CustomerRemorseReturnFeesPropertyIri;
impl PartialEq<&str> for CustomerRemorseReturnFeesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_IRI_HTTP
			|| *other == CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CustomerRemorseReturnFeesPropertyIri> for &str {
	fn eq(&self, other: &CustomerRemorseReturnFeesPropertyIri) -> bool {
		*self == CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_IRI_HTTP
			|| *self == CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_IRI_HTTPS
	}
}
pub struct CustomerRemorseReturnFeesPropertyIriOrLabel;
impl PartialEq<&str> for CustomerRemorseReturnFeesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CustomerRemorseReturnFeesPropertyIri
			|| *other == CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_LABEL
	}
}
impl PartialEq<CustomerRemorseReturnFeesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CustomerRemorseReturnFeesPropertyIriOrLabel) -> bool {
		*self == CustomerRemorseReturnFeesPropertyIri
			|| *self == CUSTOMER_REMORSE_RETURN_FEES_PROPERTY_LABEL
	}
}
