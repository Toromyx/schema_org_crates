/// <https://schema.org/fundedItem>
pub const FUNDED_ITEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/fundedItem";
/// <https://schema.org/fundedItem>
pub const FUNDED_ITEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fundedItem";
/// <https://schema.org/fundedItem>
pub const FUNDED_ITEM_PROPERTY_LABEL: &str = "fundedItem";
pub struct FundedItemPropertyIri;
impl PartialEq<&str> for FundedItemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUNDED_ITEM_PROPERTY_IRI_HTTP || *other == FUNDED_ITEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FundedItemPropertyIri> for &str {
	fn eq(&self, other: &FundedItemPropertyIri) -> bool {
		*self == FUNDED_ITEM_PROPERTY_IRI_HTTP || *self == FUNDED_ITEM_PROPERTY_IRI_HTTPS
	}
}
pub struct FundedItemPropertyIriOrLabel;
impl PartialEq<&str> for FundedItemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FundedItemPropertyIri || *other == FUNDED_ITEM_PROPERTY_LABEL
	}
}
impl PartialEq<FundedItemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FundedItemPropertyIriOrLabel) -> bool {
		*self == FundedItemPropertyIri || *self == FUNDED_ITEM_PROPERTY_LABEL
	}
}
