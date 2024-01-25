/// <https://schema.org/itemDefectReturnShippingFeesAmount>
pub const ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/itemDefectReturnShippingFeesAmount";
/// <https://schema.org/itemDefectReturnShippingFeesAmount>
pub const ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/itemDefectReturnShippingFeesAmount";
/// <https://schema.org/itemDefectReturnShippingFeesAmount>
pub const ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL: &str =
	"itemDefectReturnShippingFeesAmount";
pub struct ItemDefectReturnShippingFeesAmountPropertyIri;
impl PartialEq<&str> for ItemDefectReturnShippingFeesAmountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP
			|| *other == ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemDefectReturnShippingFeesAmountPropertyIri> for &str {
	fn eq(&self, other: &ItemDefectReturnShippingFeesAmountPropertyIri) -> bool {
		*self == ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP
			|| *self == ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemDefectReturnShippingFeesAmountPropertyIriOrLabel;
impl PartialEq<&str> for ItemDefectReturnShippingFeesAmountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemDefectReturnShippingFeesAmountPropertyIri
			|| *other == ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL
	}
}
impl PartialEq<ItemDefectReturnShippingFeesAmountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemDefectReturnShippingFeesAmountPropertyIriOrLabel) -> bool {
		*self == ItemDefectReturnShippingFeesAmountPropertyIri
			|| *self == ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_LABEL
	}
}
