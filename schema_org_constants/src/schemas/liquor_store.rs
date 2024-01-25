/// <https://schema.org/LiquorStore>
pub const LIQUOR_STORE_IRI_HTTP: &str = "http://schema.org/LiquorStore";
/// <https://schema.org/LiquorStore>
pub const LIQUOR_STORE_IRI_HTTPS: &str = "https://schema.org/LiquorStore";
/// <https://schema.org/LiquorStore>
pub const LIQUOR_STORE_LABEL: &str = "LiquorStore";
pub struct LiquorStoreIri;
impl PartialEq<&str> for LiquorStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIQUOR_STORE_IRI_HTTP || *other == LIQUOR_STORE_IRI_HTTPS
	}
}
impl PartialEq<LiquorStoreIri> for &str {
	fn eq(&self, other: &LiquorStoreIri) -> bool {
		*self == LIQUOR_STORE_IRI_HTTP || *self == LIQUOR_STORE_IRI_HTTPS
	}
}
pub struct LiquorStoreIriOrLabel;
impl PartialEq<&str> for LiquorStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LiquorStoreIri || *other == LIQUOR_STORE_LABEL
	}
}
impl PartialEq<LiquorStoreIriOrLabel> for &str {
	fn eq(&self, other: &LiquorStoreIriOrLabel) -> bool {
		*self == LiquorStoreIri || *self == LIQUOR_STORE_LABEL
	}
}
