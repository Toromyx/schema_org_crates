/// <https://schema.org/IceCreamShop>
pub const ICE_CREAM_SHOP_IRI_HTTP: &str = "http://schema.org/IceCreamShop";
/// <https://schema.org/IceCreamShop>
pub const ICE_CREAM_SHOP_IRI_HTTPS: &str = "https://schema.org/IceCreamShop";
/// <https://schema.org/IceCreamShop>
pub const ICE_CREAM_SHOP_LABEL: &str = "IceCreamShop";
pub struct IceCreamShopIri;
impl PartialEq<&str> for IceCreamShopIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ICE_CREAM_SHOP_IRI_HTTP || *other == ICE_CREAM_SHOP_IRI_HTTPS
	}
}
impl PartialEq<IceCreamShopIri> for &str {
	fn eq(&self, other: &IceCreamShopIri) -> bool {
		*self == ICE_CREAM_SHOP_IRI_HTTP || *self == ICE_CREAM_SHOP_IRI_HTTPS
	}
}
pub struct IceCreamShopIriOrLabel;
impl PartialEq<&str> for IceCreamShopIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IceCreamShopIri || *other == ICE_CREAM_SHOP_LABEL
	}
}
impl PartialEq<IceCreamShopIriOrLabel> for &str {
	fn eq(&self, other: &IceCreamShopIriOrLabel) -> bool {
		*self == IceCreamShopIri || *self == ICE_CREAM_SHOP_LABEL
	}
}
