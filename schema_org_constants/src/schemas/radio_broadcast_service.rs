/// <https://schema.org/RadioBroadcastService>
pub const RADIO_BROADCAST_SERVICE_IRI_HTTP: &str = "http://schema.org/RadioBroadcastService";
/// <https://schema.org/RadioBroadcastService>
pub const RADIO_BROADCAST_SERVICE_IRI_HTTPS: &str = "https://schema.org/RadioBroadcastService";
/// <https://schema.org/RadioBroadcastService>
pub const RADIO_BROADCAST_SERVICE_LABEL: &str = "RadioBroadcastService";
pub struct RadioBroadcastServiceIri;
impl PartialEq<&str> for RadioBroadcastServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIO_BROADCAST_SERVICE_IRI_HTTP || *other == RADIO_BROADCAST_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<RadioBroadcastServiceIri> for &str {
	fn eq(&self, other: &RadioBroadcastServiceIri) -> bool {
		*self == RADIO_BROADCAST_SERVICE_IRI_HTTP || *self == RADIO_BROADCAST_SERVICE_IRI_HTTPS
	}
}
pub struct RadioBroadcastServiceIriOrLabel;
impl PartialEq<&str> for RadioBroadcastServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadioBroadcastServiceIri || *other == RADIO_BROADCAST_SERVICE_LABEL
	}
}
impl PartialEq<RadioBroadcastServiceIriOrLabel> for &str {
	fn eq(&self, other: &RadioBroadcastServiceIriOrLabel) -> bool {
		*self == RadioBroadcastServiceIri || *self == RADIO_BROADCAST_SERVICE_LABEL
	}
}
