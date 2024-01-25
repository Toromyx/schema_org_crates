/// <https://schema.org/GroceryStore>
pub const GROCERY_STORE_IRI_HTTP: &str = "http://schema.org/GroceryStore";
/// <https://schema.org/GroceryStore>
pub const GROCERY_STORE_IRI_HTTPS: &str = "https://schema.org/GroceryStore";
/// <https://schema.org/GroceryStore>
pub const GROCERY_STORE_LABEL: &str = "GroceryStore";
pub struct GroceryStoreIri;
impl PartialEq<&str> for GroceryStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GROCERY_STORE_IRI_HTTP || *other == GROCERY_STORE_IRI_HTTPS
	}
}
impl PartialEq<GroceryStoreIri> for &str {
	fn eq(&self, other: &GroceryStoreIri) -> bool {
		*self == GROCERY_STORE_IRI_HTTP || *self == GROCERY_STORE_IRI_HTTPS
	}
}
pub struct GroceryStoreIriOrLabel;
impl PartialEq<&str> for GroceryStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GroceryStoreIri || *other == GROCERY_STORE_LABEL
	}
}
impl PartialEq<GroceryStoreIriOrLabel> for &str {
	fn eq(&self, other: &GroceryStoreIriOrLabel) -> bool {
		*self == GroceryStoreIri || *self == GROCERY_STORE_LABEL
	}
}
