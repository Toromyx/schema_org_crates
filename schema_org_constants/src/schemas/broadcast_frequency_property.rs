/// <https://schema.org/broadcastFrequency>
pub const BROADCAST_FREQUENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcastFrequency";
/// <https://schema.org/broadcastFrequency>
pub const BROADCAST_FREQUENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/broadcastFrequency";
/// <https://schema.org/broadcastFrequency>
pub const BROADCAST_FREQUENCY_PROPERTY_LABEL: &str = "broadcastFrequency";
pub struct BroadcastFrequencyPropertyIri;
impl PartialEq<&str> for BroadcastFrequencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_FREQUENCY_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastFrequencyPropertyIri> for &str {
	fn eq(&self, other: &BroadcastFrequencyPropertyIri) -> bool {
		*self == BROADCAST_FREQUENCY_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastFrequencyPropertyIriOrLabel;
impl PartialEq<&str> for BroadcastFrequencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastFrequencyPropertyIri || *other == BROADCAST_FREQUENCY_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastFrequencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastFrequencyPropertyIriOrLabel) -> bool {
		*self == BroadcastFrequencyPropertyIri || *self == BROADCAST_FREQUENCY_PROPERTY_LABEL
	}
}
