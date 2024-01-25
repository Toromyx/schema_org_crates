/// <https://schema.org/itemCondition>
pub const ITEM_CONDITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/itemCondition";
/// <https://schema.org/itemCondition>
pub const ITEM_CONDITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/itemCondition";
/// <https://schema.org/itemCondition>
pub const ITEM_CONDITION_PROPERTY_LABEL: &str = "itemCondition";
pub struct ItemConditionPropertyIri;
impl PartialEq<&str> for ItemConditionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_CONDITION_PROPERTY_IRI_HTTP || *other == ITEM_CONDITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemConditionPropertyIri> for &str {
	fn eq(&self, other: &ItemConditionPropertyIri) -> bool {
		*self == ITEM_CONDITION_PROPERTY_IRI_HTTP || *self == ITEM_CONDITION_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemConditionPropertyIriOrLabel;
impl PartialEq<&str> for ItemConditionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemConditionPropertyIri || *other == ITEM_CONDITION_PROPERTY_LABEL
	}
}
impl PartialEq<ItemConditionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemConditionPropertyIriOrLabel) -> bool {
		*self == ItemConditionPropertyIri || *self == ITEM_CONDITION_PROPERTY_LABEL
	}
}
