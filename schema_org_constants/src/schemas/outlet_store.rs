/// <https://schema.org/OutletStore>
pub const OUTLET_STORE_IRI_HTTP: &str = "http://schema.org/OutletStore";
/// <https://schema.org/OutletStore>
pub const OUTLET_STORE_IRI_HTTPS: &str = "https://schema.org/OutletStore";
/// <https://schema.org/OutletStore>
pub const OUTLET_STORE_LABEL: &str = "OutletStore";
pub struct OutletStoreIri;
impl PartialEq<&str> for OutletStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OUTLET_STORE_IRI_HTTP || *other == OUTLET_STORE_IRI_HTTPS
	}
}
impl PartialEq<OutletStoreIri> for &str {
	fn eq(&self, other: &OutletStoreIri) -> bool {
		*self == OUTLET_STORE_IRI_HTTP || *self == OUTLET_STORE_IRI_HTTPS
	}
}
pub struct OutletStoreIriOrLabel;
impl PartialEq<&str> for OutletStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OutletStoreIri || *other == OUTLET_STORE_LABEL
	}
}
impl PartialEq<OutletStoreIriOrLabel> for &str {
	fn eq(&self, other: &OutletStoreIriOrLabel) -> bool {
		*self == OutletStoreIri || *self == OUTLET_STORE_LABEL
	}
}
