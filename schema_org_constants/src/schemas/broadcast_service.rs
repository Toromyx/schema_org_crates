/// <https://schema.org/BroadcastService>
pub const BROADCAST_SERVICE_IRI_HTTP: &str = "http://schema.org/BroadcastService";
/// <https://schema.org/BroadcastService>
pub const BROADCAST_SERVICE_IRI_HTTPS: &str = "https://schema.org/BroadcastService";
/// <https://schema.org/BroadcastService>
pub const BROADCAST_SERVICE_LABEL: &str = "BroadcastService";
pub struct BroadcastServiceIri;
impl PartialEq<&str> for BroadcastServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_SERVICE_IRI_HTTP || *other == BROADCAST_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<BroadcastServiceIri> for &str {
	fn eq(&self, other: &BroadcastServiceIri) -> bool {
		*self == BROADCAST_SERVICE_IRI_HTTP || *self == BROADCAST_SERVICE_IRI_HTTPS
	}
}
pub struct BroadcastServiceIriOrLabel;
impl PartialEq<&str> for BroadcastServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastServiceIri || *other == BROADCAST_SERVICE_LABEL
	}
}
impl PartialEq<BroadcastServiceIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastServiceIriOrLabel) -> bool {
		*self == BroadcastServiceIri || *self == BROADCAST_SERVICE_LABEL
	}
}
