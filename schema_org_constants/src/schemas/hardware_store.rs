/// <https://schema.org/HardwareStore>
pub const HARDWARE_STORE_IRI_HTTP: &str = "http://schema.org/HardwareStore";
/// <https://schema.org/HardwareStore>
pub const HARDWARE_STORE_IRI_HTTPS: &str = "https://schema.org/HardwareStore";
/// <https://schema.org/HardwareStore>
pub const HARDWARE_STORE_LABEL: &str = "HardwareStore";
pub struct HardwareStoreIri;
impl PartialEq<&str> for HardwareStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HARDWARE_STORE_IRI_HTTP || *other == HARDWARE_STORE_IRI_HTTPS
	}
}
impl PartialEq<HardwareStoreIri> for &str {
	fn eq(&self, other: &HardwareStoreIri) -> bool {
		*self == HARDWARE_STORE_IRI_HTTP || *self == HARDWARE_STORE_IRI_HTTPS
	}
}
pub struct HardwareStoreIriOrLabel;
impl PartialEq<&str> for HardwareStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HardwareStoreIri || *other == HARDWARE_STORE_LABEL
	}
}
impl PartialEq<HardwareStoreIriOrLabel> for &str {
	fn eq(&self, other: &HardwareStoreIriOrLabel) -> bool {
		*self == HardwareStoreIri || *self == HARDWARE_STORE_LABEL
	}
}
