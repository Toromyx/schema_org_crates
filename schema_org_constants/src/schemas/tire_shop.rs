/// <https://schema.org/TireShop>
pub const TIRE_SHOP_IRI_HTTP: &str = "http://schema.org/TireShop";
/// <https://schema.org/TireShop>
pub const TIRE_SHOP_IRI_HTTPS: &str = "https://schema.org/TireShop";
/// <https://schema.org/TireShop>
pub const TIRE_SHOP_LABEL: &str = "TireShop";
pub struct TireShopIri;
impl PartialEq<&str> for TireShopIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TIRE_SHOP_IRI_HTTP || *other == TIRE_SHOP_IRI_HTTPS
	}
}
impl PartialEq<TireShopIri> for &str {
	fn eq(&self, other: &TireShopIri) -> bool {
		*self == TIRE_SHOP_IRI_HTTP || *self == TIRE_SHOP_IRI_HTTPS
	}
}
pub struct TireShopIriOrLabel;
impl PartialEq<&str> for TireShopIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TireShopIri || *other == TIRE_SHOP_LABEL
	}
}
impl PartialEq<TireShopIriOrLabel> for &str {
	fn eq(&self, other: &TireShopIriOrLabel) -> bool {
		*self == TireShopIri || *self == TIRE_SHOP_LABEL
	}
}
