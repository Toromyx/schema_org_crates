/// <https://schema.org/broadcaster>
pub const BROADCASTER_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcaster";
/// <https://schema.org/broadcaster>
pub const BROADCASTER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/broadcaster";
/// <https://schema.org/broadcaster>
pub const BROADCASTER_PROPERTY_LABEL: &str = "broadcaster";
pub struct BroadcasterPropertyIri;
impl PartialEq<&str> for BroadcasterPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCASTER_PROPERTY_IRI_HTTP || *other == BROADCASTER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcasterPropertyIri> for &str {
	fn eq(&self, other: &BroadcasterPropertyIri) -> bool {
		*self == BROADCASTER_PROPERTY_IRI_HTTP || *self == BROADCASTER_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcasterPropertyIriOrLabel;
impl PartialEq<&str> for BroadcasterPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcasterPropertyIri || *other == BROADCASTER_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcasterPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcasterPropertyIriOrLabel) -> bool {
		*self == BroadcasterPropertyIri || *self == BROADCASTER_PROPERTY_LABEL
	}
}
