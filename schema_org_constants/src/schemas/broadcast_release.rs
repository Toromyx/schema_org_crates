/// <https://schema.org/BroadcastRelease>
pub const BROADCAST_RELEASE_IRI_HTTP: &str = "http://schema.org/BroadcastRelease";
/// <https://schema.org/BroadcastRelease>
pub const BROADCAST_RELEASE_IRI_HTTPS: &str = "https://schema.org/BroadcastRelease";
/// <https://schema.org/BroadcastRelease>
pub const BROADCAST_RELEASE_LABEL: &str = "BroadcastRelease";
pub struct BroadcastReleaseIri;
impl PartialEq<&str> for BroadcastReleaseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_RELEASE_IRI_HTTP || *other == BROADCAST_RELEASE_IRI_HTTPS
	}
}
impl PartialEq<BroadcastReleaseIri> for &str {
	fn eq(&self, other: &BroadcastReleaseIri) -> bool {
		*self == BROADCAST_RELEASE_IRI_HTTP || *self == BROADCAST_RELEASE_IRI_HTTPS
	}
}
pub struct BroadcastReleaseIriOrLabel;
impl PartialEq<&str> for BroadcastReleaseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastReleaseIri || *other == BROADCAST_RELEASE_LABEL
	}
}
impl PartialEq<BroadcastReleaseIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastReleaseIriOrLabel) -> bool {
		*self == BroadcastReleaseIri || *self == BROADCAST_RELEASE_LABEL
	}
}
