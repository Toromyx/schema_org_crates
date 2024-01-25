/// <https://schema.org/CafeOrCoffeeShop>
pub const CAFE_OR_COFFEE_SHOP_IRI_HTTP: &str = "http://schema.org/CafeOrCoffeeShop";
/// <https://schema.org/CafeOrCoffeeShop>
pub const CAFE_OR_COFFEE_SHOP_IRI_HTTPS: &str = "https://schema.org/CafeOrCoffeeShop";
/// <https://schema.org/CafeOrCoffeeShop>
pub const CAFE_OR_COFFEE_SHOP_LABEL: &str = "CafeOrCoffeeShop";
pub struct CafeOrCoffeeShopIri;
impl PartialEq<&str> for CafeOrCoffeeShopIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CAFE_OR_COFFEE_SHOP_IRI_HTTP || *other == CAFE_OR_COFFEE_SHOP_IRI_HTTPS
	}
}
impl PartialEq<CafeOrCoffeeShopIri> for &str {
	fn eq(&self, other: &CafeOrCoffeeShopIri) -> bool {
		*self == CAFE_OR_COFFEE_SHOP_IRI_HTTP || *self == CAFE_OR_COFFEE_SHOP_IRI_HTTPS
	}
}
pub struct CafeOrCoffeeShopIriOrLabel;
impl PartialEq<&str> for CafeOrCoffeeShopIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CafeOrCoffeeShopIri || *other == CAFE_OR_COFFEE_SHOP_LABEL
	}
}
impl PartialEq<CafeOrCoffeeShopIriOrLabel> for &str {
	fn eq(&self, other: &CafeOrCoffeeShopIriOrLabel) -> bool {
		*self == CafeOrCoffeeShopIri || *self == CAFE_OR_COFFEE_SHOP_LABEL
	}
}
