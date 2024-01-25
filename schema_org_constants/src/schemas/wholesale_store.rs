/// <https://schema.org/WholesaleStore>
pub const WHOLESALE_STORE_IRI_HTTP: &str = "http://schema.org/WholesaleStore";
/// <https://schema.org/WholesaleStore>
pub const WHOLESALE_STORE_IRI_HTTPS: &str = "https://schema.org/WholesaleStore";
/// <https://schema.org/WholesaleStore>
pub const WHOLESALE_STORE_LABEL: &str = "WholesaleStore";
pub struct WholesaleStoreIri;
impl PartialEq<&str> for WholesaleStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WHOLESALE_STORE_IRI_HTTP || *other == WHOLESALE_STORE_IRI_HTTPS
	}
}
impl PartialEq<WholesaleStoreIri> for &str {
	fn eq(&self, other: &WholesaleStoreIri) -> bool {
		*self == WHOLESALE_STORE_IRI_HTTP || *self == WHOLESALE_STORE_IRI_HTTPS
	}
}
pub struct WholesaleStoreIriOrLabel;
impl PartialEq<&str> for WholesaleStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WholesaleStoreIri || *other == WHOLESALE_STORE_LABEL
	}
}
impl PartialEq<WholesaleStoreIriOrLabel> for &str {
	fn eq(&self, other: &WholesaleStoreIriOrLabel) -> bool {
		*self == WholesaleStoreIri || *self == WHOLESALE_STORE_LABEL
	}
}
