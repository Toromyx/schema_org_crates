/// <https://schema.org/AutoBodyShop>
pub const AUTO_BODY_SHOP_IRI_HTTP: &str = "http://schema.org/AutoBodyShop";
/// <https://schema.org/AutoBodyShop>
pub const AUTO_BODY_SHOP_IRI_HTTPS: &str = "https://schema.org/AutoBodyShop";
/// <https://schema.org/AutoBodyShop>
pub const AUTO_BODY_SHOP_LABEL: &str = "AutoBodyShop";
pub struct AutoBodyShopIri;
impl PartialEq<&str> for AutoBodyShopIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTO_BODY_SHOP_IRI_HTTP || *other == AUTO_BODY_SHOP_IRI_HTTPS
	}
}
impl PartialEq<AutoBodyShopIri> for &str {
	fn eq(&self, other: &AutoBodyShopIri) -> bool {
		*self == AUTO_BODY_SHOP_IRI_HTTP || *self == AUTO_BODY_SHOP_IRI_HTTPS
	}
}
pub struct AutoBodyShopIriOrLabel;
impl PartialEq<&str> for AutoBodyShopIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AutoBodyShopIri || *other == AUTO_BODY_SHOP_LABEL
	}
}
impl PartialEq<AutoBodyShopIriOrLabel> for &str {
	fn eq(&self, other: &AutoBodyShopIriOrLabel) -> bool {
		*self == AutoBodyShopIri || *self == AUTO_BODY_SHOP_LABEL
	}
}
