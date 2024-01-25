/// <https://schema.org/OnlineStore>
pub const ONLINE_STORE_IRI_HTTP: &str = "http://schema.org/OnlineStore";
/// <https://schema.org/OnlineStore>
pub const ONLINE_STORE_IRI_HTTPS: &str = "https://schema.org/OnlineStore";
/// <https://schema.org/OnlineStore>
pub const ONLINE_STORE_LABEL: &str = "OnlineStore";
pub struct OnlineStoreIri;
impl PartialEq<&str> for OnlineStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ONLINE_STORE_IRI_HTTP || *other == ONLINE_STORE_IRI_HTTPS
	}
}
impl PartialEq<OnlineStoreIri> for &str {
	fn eq(&self, other: &OnlineStoreIri) -> bool {
		*self == ONLINE_STORE_IRI_HTTP || *self == ONLINE_STORE_IRI_HTTPS
	}
}
pub struct OnlineStoreIriOrLabel;
impl PartialEq<&str> for OnlineStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OnlineStoreIri || *other == ONLINE_STORE_LABEL
	}
}
impl PartialEq<OnlineStoreIriOrLabel> for &str {
	fn eq(&self, other: &OnlineStoreIriOrLabel) -> bool {
		*self == OnlineStoreIri || *self == ONLINE_STORE_LABEL
	}
}
