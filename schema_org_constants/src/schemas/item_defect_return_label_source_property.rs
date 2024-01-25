/// <https://schema.org/itemDefectReturnLabelSource>
pub const ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/itemDefectReturnLabelSource";
/// <https://schema.org/itemDefectReturnLabelSource>
pub const ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/itemDefectReturnLabelSource";
/// <https://schema.org/itemDefectReturnLabelSource>
pub const ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_LABEL: &str = "itemDefectReturnLabelSource";
pub struct ItemDefectReturnLabelSourcePropertyIri;
impl PartialEq<&str> for ItemDefectReturnLabelSourcePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP
			|| *other == ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemDefectReturnLabelSourcePropertyIri> for &str {
	fn eq(&self, other: &ItemDefectReturnLabelSourcePropertyIri) -> bool {
		*self == ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP
			|| *self == ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemDefectReturnLabelSourcePropertyIriOrLabel;
impl PartialEq<&str> for ItemDefectReturnLabelSourcePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemDefectReturnLabelSourcePropertyIri
			|| *other == ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_LABEL
	}
}
impl PartialEq<ItemDefectReturnLabelSourcePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemDefectReturnLabelSourcePropertyIriOrLabel) -> bool {
		*self == ItemDefectReturnLabelSourcePropertyIri
			|| *self == ITEM_DEFECT_RETURN_LABEL_SOURCE_PROPERTY_LABEL
	}
}
