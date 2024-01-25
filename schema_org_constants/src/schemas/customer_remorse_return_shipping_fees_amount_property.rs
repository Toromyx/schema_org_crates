/// <https://schema.org/customerRemorseReturnShippingFeesAmount>
pub const CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/customerRemorseReturnShippingFeesAmount";
/// <https://schema.org/customerRemorseReturnShippingFeesAmount>
pub const CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/customerRemorseReturnShippingFeesAmount";
/// <https://schema.org/customerRemorseReturnShippingFeesAmount>
pub const CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL: &str =
	"customerRemorseReturnShippingFeesAmount";
pub struct CustomerRemorseReturnShippingFeesAmountPropertyIri;
impl PartialEq<&str> for CustomerRemorseReturnShippingFeesAmountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP
			|| *other == CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CustomerRemorseReturnShippingFeesAmountPropertyIri> for &str {
	fn eq(&self, other: &CustomerRemorseReturnShippingFeesAmountPropertyIri) -> bool {
		*self == CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP
			|| *self == CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct CustomerRemorseReturnShippingFeesAmountPropertyIriOrLabel;
impl PartialEq<&str> for CustomerRemorseReturnShippingFeesAmountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CustomerRemorseReturnShippingFeesAmountPropertyIri
			|| *other == CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL
	}
}
impl PartialEq<CustomerRemorseReturnShippingFeesAmountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CustomerRemorseReturnShippingFeesAmountPropertyIriOrLabel) -> bool {
		*self == CustomerRemorseReturnShippingFeesAmountPropertyIri
			|| *self == CUSTOMER_REMORSE_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL
	}
}
