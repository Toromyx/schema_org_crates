/// <https://schema.org/ToyStore>
pub const TOY_STORE_IRI_HTTP: &str = "http://schema.org/ToyStore";
/// <https://schema.org/ToyStore>
pub const TOY_STORE_IRI_HTTPS: &str = "https://schema.org/ToyStore";
/// <https://schema.org/ToyStore>
pub const TOY_STORE_LABEL: &str = "ToyStore";
pub struct ToyStoreIri;
impl PartialEq<&str> for ToyStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOY_STORE_IRI_HTTP || *other == TOY_STORE_IRI_HTTPS
	}
}
impl PartialEq<ToyStoreIri> for &str {
	fn eq(&self, other: &ToyStoreIri) -> bool {
		*self == TOY_STORE_IRI_HTTP || *self == TOY_STORE_IRI_HTTPS
	}
}
pub struct ToyStoreIriOrLabel;
impl PartialEq<&str> for ToyStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ToyStoreIri || *other == TOY_STORE_LABEL
	}
}
impl PartialEq<ToyStoreIriOrLabel> for &str {
	fn eq(&self, other: &ToyStoreIriOrLabel) -> bool {
		*self == ToyStoreIri || *self == TOY_STORE_LABEL
	}
}
