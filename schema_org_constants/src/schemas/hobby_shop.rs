/// <https://schema.org/HobbyShop>
pub const HOBBY_SHOP_IRI_HTTP: &str = "http://schema.org/HobbyShop";
/// <https://schema.org/HobbyShop>
pub const HOBBY_SHOP_IRI_HTTPS: &str = "https://schema.org/HobbyShop";
/// <https://schema.org/HobbyShop>
pub const HOBBY_SHOP_LABEL: &str = "HobbyShop";
pub struct HobbyShopIri;
impl PartialEq<&str> for HobbyShopIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOBBY_SHOP_IRI_HTTP || *other == HOBBY_SHOP_IRI_HTTPS
	}
}
impl PartialEq<HobbyShopIri> for &str {
	fn eq(&self, other: &HobbyShopIri) -> bool {
		*self == HOBBY_SHOP_IRI_HTTP || *self == HOBBY_SHOP_IRI_HTTPS
	}
}
pub struct HobbyShopIriOrLabel;
impl PartialEq<&str> for HobbyShopIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HobbyShopIri || *other == HOBBY_SHOP_LABEL
	}
}
impl PartialEq<HobbyShopIriOrLabel> for &str {
	fn eq(&self, other: &HobbyShopIriOrLabel) -> bool {
		*self == HobbyShopIri || *self == HOBBY_SHOP_LABEL
	}
}
