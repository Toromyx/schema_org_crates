/// <https://schema.org/Bridge>
pub const BRIDGE_IRI_HTTP: &str = "http://schema.org/Bridge";
/// <https://schema.org/Bridge>
pub const BRIDGE_IRI_HTTPS: &str = "https://schema.org/Bridge";
/// <https://schema.org/Bridge>
pub const BRIDGE_LABEL: &str = "Bridge";
pub struct BridgeIri;
impl PartialEq<&str> for BridgeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BRIDGE_IRI_HTTP || *other == BRIDGE_IRI_HTTPS
	}
}
impl PartialEq<BridgeIri> for &str {
	fn eq(&self, other: &BridgeIri) -> bool {
		*self == BRIDGE_IRI_HTTP || *self == BRIDGE_IRI_HTTPS
	}
}
pub struct BridgeIriOrLabel;
impl PartialEq<&str> for BridgeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BridgeIri || *other == BRIDGE_LABEL
	}
}
impl PartialEq<BridgeIriOrLabel> for &str {
	fn eq(&self, other: &BridgeIriOrLabel) -> bool {
		*self == BridgeIri || *self == BRIDGE_LABEL
	}
}
