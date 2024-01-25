/// <https://schema.org/Store>
pub const STORE_IRI_HTTP: &str = "http://schema.org/Store";
/// <https://schema.org/Store>
pub const STORE_IRI_HTTPS: &str = "https://schema.org/Store";
/// <https://schema.org/Store>
pub const STORE_LABEL: &str = "Store";
pub struct StoreIri;
impl PartialEq<&str> for StoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STORE_IRI_HTTP || *other == STORE_IRI_HTTPS
	}
}
impl PartialEq<StoreIri> for &str {
	fn eq(&self, other: &StoreIri) -> bool {
		*self == STORE_IRI_HTTP || *self == STORE_IRI_HTTPS
	}
}
pub struct StoreIriOrLabel;
impl PartialEq<&str> for StoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StoreIri || *other == STORE_LABEL
	}
}
impl PartialEq<StoreIriOrLabel> for &str {
	fn eq(&self, other: &StoreIriOrLabel) -> bool {
		*self == StoreIri || *self == STORE_LABEL
	}
}
