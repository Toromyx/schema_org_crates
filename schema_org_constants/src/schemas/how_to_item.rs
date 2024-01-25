/// <https://schema.org/HowToItem>
pub const HOW_TO_ITEM_IRI_HTTP: &str = "http://schema.org/HowToItem";
/// <https://schema.org/HowToItem>
pub const HOW_TO_ITEM_IRI_HTTPS: &str = "https://schema.org/HowToItem";
/// <https://schema.org/HowToItem>
pub const HOW_TO_ITEM_LABEL: &str = "HowToItem";
pub struct HowToItemIri;
impl PartialEq<&str> for HowToItemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_TO_ITEM_IRI_HTTP || *other == HOW_TO_ITEM_IRI_HTTPS
	}
}
impl PartialEq<HowToItemIri> for &str {
	fn eq(&self, other: &HowToItemIri) -> bool {
		*self == HOW_TO_ITEM_IRI_HTTP || *self == HOW_TO_ITEM_IRI_HTTPS
	}
}
pub struct HowToItemIriOrLabel;
impl PartialEq<&str> for HowToItemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowToItemIri || *other == HOW_TO_ITEM_LABEL
	}
}
impl PartialEq<HowToItemIriOrLabel> for &str {
	fn eq(&self, other: &HowToItemIriOrLabel) -> bool {
		*self == HowToItemIri || *self == HOW_TO_ITEM_LABEL
	}
}
