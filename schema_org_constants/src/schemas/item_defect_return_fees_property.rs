/// <https://schema.org/itemDefectReturnFees>
pub const ITEM_DEFECT_RETURN_FEES_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/itemDefectReturnFees";
/// <https://schema.org/itemDefectReturnFees>
pub const ITEM_DEFECT_RETURN_FEES_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/itemDefectReturnFees";
/// <https://schema.org/itemDefectReturnFees>
pub const ITEM_DEFECT_RETURN_FEES_PROPERTY_LABEL: &str = "itemDefectReturnFees";
pub struct ItemDefectReturnFeesPropertyIri;
impl PartialEq<&str> for ItemDefectReturnFeesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_DEFECT_RETURN_FEES_PROPERTY_IRI_HTTP
			|| *other == ITEM_DEFECT_RETURN_FEES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemDefectReturnFeesPropertyIri> for &str {
	fn eq(&self, other: &ItemDefectReturnFeesPropertyIri) -> bool {
		*self == ITEM_DEFECT_RETURN_FEES_PROPERTY_IRI_HTTP
			|| *self == ITEM_DEFECT_RETURN_FEES_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemDefectReturnFeesPropertyIriOrLabel;
impl PartialEq<&str> for ItemDefectReturnFeesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemDefectReturnFeesPropertyIri
			|| *other == ITEM_DEFECT_RETURN_FEES_PROPERTY_LABEL
	}
}
impl PartialEq<ItemDefectReturnFeesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemDefectReturnFeesPropertyIriOrLabel) -> bool {
		*self == ItemDefectReturnFeesPropertyIri || *self == ITEM_DEFECT_RETURN_FEES_PROPERTY_LABEL
	}
}
