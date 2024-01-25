/// <https://schema.org/ConvenienceStore>
pub const CONVENIENCE_STORE_IRI_HTTP: &str = "http://schema.org/ConvenienceStore";
/// <https://schema.org/ConvenienceStore>
pub const CONVENIENCE_STORE_IRI_HTTPS: &str = "https://schema.org/ConvenienceStore";
/// <https://schema.org/ConvenienceStore>
pub const CONVENIENCE_STORE_LABEL: &str = "ConvenienceStore";
pub struct ConvenienceStoreIri;
impl PartialEq<&str> for ConvenienceStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONVENIENCE_STORE_IRI_HTTP || *other == CONVENIENCE_STORE_IRI_HTTPS
	}
}
impl PartialEq<ConvenienceStoreIri> for &str {
	fn eq(&self, other: &ConvenienceStoreIri) -> bool {
		*self == CONVENIENCE_STORE_IRI_HTTP || *self == CONVENIENCE_STORE_IRI_HTTPS
	}
}
pub struct ConvenienceStoreIriOrLabel;
impl PartialEq<&str> for ConvenienceStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConvenienceStoreIri || *other == CONVENIENCE_STORE_LABEL
	}
}
impl PartialEq<ConvenienceStoreIriOrLabel> for &str {
	fn eq(&self, other: &ConvenienceStoreIriOrLabel) -> bool {
		*self == ConvenienceStoreIri || *self == CONVENIENCE_STORE_LABEL
	}
}
