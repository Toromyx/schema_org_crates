/// <https://schema.org/HomeGoodsStore>
pub const HOME_GOODS_STORE_IRI_HTTP: &str = "http://schema.org/HomeGoodsStore";
/// <https://schema.org/HomeGoodsStore>
pub const HOME_GOODS_STORE_IRI_HTTPS: &str = "https://schema.org/HomeGoodsStore";
/// <https://schema.org/HomeGoodsStore>
pub const HOME_GOODS_STORE_LABEL: &str = "HomeGoodsStore";
pub struct HomeGoodsStoreIri;
impl PartialEq<&str> for HomeGoodsStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOME_GOODS_STORE_IRI_HTTP || *other == HOME_GOODS_STORE_IRI_HTTPS
	}
}
impl PartialEq<HomeGoodsStoreIri> for &str {
	fn eq(&self, other: &HomeGoodsStoreIri) -> bool {
		*self == HOME_GOODS_STORE_IRI_HTTP || *self == HOME_GOODS_STORE_IRI_HTTPS
	}
}
pub struct HomeGoodsStoreIriOrLabel;
impl PartialEq<&str> for HomeGoodsStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HomeGoodsStoreIri || *other == HOME_GOODS_STORE_LABEL
	}
}
impl PartialEq<HomeGoodsStoreIriOrLabel> for &str {
	fn eq(&self, other: &HomeGoodsStoreIriOrLabel) -> bool {
		*self == HomeGoodsStoreIri || *self == HOME_GOODS_STORE_LABEL
	}
}
