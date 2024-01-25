/// <https://schema.org/SportingGoodsStore>
pub const SPORTING_GOODS_STORE_IRI_HTTP: &str = "http://schema.org/SportingGoodsStore";
/// <https://schema.org/SportingGoodsStore>
pub const SPORTING_GOODS_STORE_IRI_HTTPS: &str = "https://schema.org/SportingGoodsStore";
/// <https://schema.org/SportingGoodsStore>
pub const SPORTING_GOODS_STORE_LABEL: &str = "SportingGoodsStore";
pub struct SportingGoodsStoreIri;
impl PartialEq<&str> for SportingGoodsStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTING_GOODS_STORE_IRI_HTTP || *other == SPORTING_GOODS_STORE_IRI_HTTPS
	}
}
impl PartialEq<SportingGoodsStoreIri> for &str {
	fn eq(&self, other: &SportingGoodsStoreIri) -> bool {
		*self == SPORTING_GOODS_STORE_IRI_HTTP || *self == SPORTING_GOODS_STORE_IRI_HTTPS
	}
}
pub struct SportingGoodsStoreIriOrLabel;
impl PartialEq<&str> for SportingGoodsStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportingGoodsStoreIri || *other == SPORTING_GOODS_STORE_LABEL
	}
}
impl PartialEq<SportingGoodsStoreIriOrLabel> for &str {
	fn eq(&self, other: &SportingGoodsStoreIriOrLabel) -> bool {
		*self == SportingGoodsStoreIri || *self == SPORTING_GOODS_STORE_LABEL
	}
}
