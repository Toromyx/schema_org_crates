/// <https://schema.org/ComputerStore>
pub const COMPUTER_STORE_IRI_HTTP: &str = "http://schema.org/ComputerStore";
/// <https://schema.org/ComputerStore>
pub const COMPUTER_STORE_IRI_HTTPS: &str = "https://schema.org/ComputerStore";
/// <https://schema.org/ComputerStore>
pub const COMPUTER_STORE_LABEL: &str = "ComputerStore";
pub struct ComputerStoreIri;
impl PartialEq<&str> for ComputerStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPUTER_STORE_IRI_HTTP || *other == COMPUTER_STORE_IRI_HTTPS
	}
}
impl PartialEq<ComputerStoreIri> for &str {
	fn eq(&self, other: &ComputerStoreIri) -> bool {
		*self == COMPUTER_STORE_IRI_HTTP || *self == COMPUTER_STORE_IRI_HTTPS
	}
}
pub struct ComputerStoreIriOrLabel;
impl PartialEq<&str> for ComputerStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComputerStoreIri || *other == COMPUTER_STORE_LABEL
	}
}
impl PartialEq<ComputerStoreIriOrLabel> for &str {
	fn eq(&self, other: &ComputerStoreIriOrLabel) -> bool {
		*self == ComputerStoreIri || *self == COMPUTER_STORE_LABEL
	}
}
