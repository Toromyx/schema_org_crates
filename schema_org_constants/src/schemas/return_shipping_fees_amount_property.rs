/// <https://schema.org/returnShippingFeesAmount>
pub const RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/returnShippingFeesAmount";
/// <https://schema.org/returnShippingFeesAmount>
pub const RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/returnShippingFeesAmount";
/// <https://schema.org/returnShippingFeesAmount>
pub const RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL: &str = "returnShippingFeesAmount";
pub struct ReturnShippingFeesAmountPropertyIri;
impl PartialEq<&str> for ReturnShippingFeesAmountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP
			|| *other == RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReturnShippingFeesAmountPropertyIri> for &str {
	fn eq(&self, other: &ReturnShippingFeesAmountPropertyIri) -> bool {
		*self == RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP
			|| *self == RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct ReturnShippingFeesAmountPropertyIriOrLabel;
impl PartialEq<&str> for ReturnShippingFeesAmountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnShippingFeesAmountPropertyIri
			|| *other == RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL
	}
}
impl PartialEq<ReturnShippingFeesAmountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReturnShippingFeesAmountPropertyIriOrLabel) -> bool {
		*self == ReturnShippingFeesAmountPropertyIri
			|| *self == RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL
	}
}
