/// <https://schema.org/ReturnInStore>
pub const RETURN_IN_STORE_IRI_HTTP: &str = "http://schema.org/ReturnInStore";
/// <https://schema.org/ReturnInStore>
pub const RETURN_IN_STORE_IRI_HTTPS: &str = "https://schema.org/ReturnInStore";
/// <https://schema.org/ReturnInStore>
pub const RETURN_IN_STORE_LABEL: &str = "ReturnInStore";
pub struct ReturnInStoreIri;
impl PartialEq<&str> for ReturnInStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_IN_STORE_IRI_HTTP || *other == RETURN_IN_STORE_IRI_HTTPS
	}
}
impl PartialEq<ReturnInStoreIri> for &str {
	fn eq(&self, other: &ReturnInStoreIri) -> bool {
		*self == RETURN_IN_STORE_IRI_HTTP || *self == RETURN_IN_STORE_IRI_HTTPS
	}
}
pub struct ReturnInStoreIriOrLabel;
impl PartialEq<&str> for ReturnInStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnInStoreIri || *other == RETURN_IN_STORE_LABEL
	}
}
impl PartialEq<ReturnInStoreIriOrLabel> for &str {
	fn eq(&self, other: &ReturnInStoreIriOrLabel) -> bool {
		*self == ReturnInStoreIri || *self == RETURN_IN_STORE_LABEL
	}
}
