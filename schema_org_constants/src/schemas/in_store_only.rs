/// <https://schema.org/InStoreOnly>
pub const IN_STORE_ONLY_IRI_HTTP: &str = "http://schema.org/InStoreOnly";
/// <https://schema.org/InStoreOnly>
pub const IN_STORE_ONLY_IRI_HTTPS: &str = "https://schema.org/InStoreOnly";
/// <https://schema.org/InStoreOnly>
pub const IN_STORE_ONLY_LABEL: &str = "InStoreOnly";
pub struct InStoreOnlyIri;
impl PartialEq<&str> for InStoreOnlyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_STORE_ONLY_IRI_HTTP || *other == IN_STORE_ONLY_IRI_HTTPS
	}
}
impl PartialEq<InStoreOnlyIri> for &str {
	fn eq(&self, other: &InStoreOnlyIri) -> bool {
		*self == IN_STORE_ONLY_IRI_HTTP || *self == IN_STORE_ONLY_IRI_HTTPS
	}
}
pub struct InStoreOnlyIriOrLabel;
impl PartialEq<&str> for InStoreOnlyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InStoreOnlyIri || *other == IN_STORE_ONLY_LABEL
	}
}
impl PartialEq<InStoreOnlyIriOrLabel> for &str {
	fn eq(&self, other: &InStoreOnlyIriOrLabel) -> bool {
		*self == InStoreOnlyIri || *self == IN_STORE_ONLY_LABEL
	}
}
