/// <https://schema.org/broadcastFrequencyValue>
pub const BROADCAST_FREQUENCY_VALUE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/broadcastFrequencyValue";
/// <https://schema.org/broadcastFrequencyValue>
pub const BROADCAST_FREQUENCY_VALUE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/broadcastFrequencyValue";
/// <https://schema.org/broadcastFrequencyValue>
pub const BROADCAST_FREQUENCY_VALUE_PROPERTY_LABEL: &str = "broadcastFrequencyValue";
pub struct BroadcastFrequencyValuePropertyIri;
impl PartialEq<&str> for BroadcastFrequencyValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_FREQUENCY_VALUE_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_FREQUENCY_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastFrequencyValuePropertyIri> for &str {
	fn eq(&self, other: &BroadcastFrequencyValuePropertyIri) -> bool {
		*self == BROADCAST_FREQUENCY_VALUE_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_FREQUENCY_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastFrequencyValuePropertyIriOrLabel;
impl PartialEq<&str> for BroadcastFrequencyValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastFrequencyValuePropertyIri
			|| *other == BROADCAST_FREQUENCY_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastFrequencyValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastFrequencyValuePropertyIriOrLabel) -> bool {
		*self == BroadcastFrequencyValuePropertyIri
			|| *self == BROADCAST_FREQUENCY_VALUE_PROPERTY_LABEL
	}
}
