/// <https://schema.org/broadcastSignalModulation>
pub const BROADCAST_SIGNAL_MODULATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/broadcastSignalModulation";
/// <https://schema.org/broadcastSignalModulation>
pub const BROADCAST_SIGNAL_MODULATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/broadcastSignalModulation";
/// <https://schema.org/broadcastSignalModulation>
pub const BROADCAST_SIGNAL_MODULATION_PROPERTY_LABEL: &str = "broadcastSignalModulation";
pub struct BroadcastSignalModulationPropertyIri;
impl PartialEq<&str> for BroadcastSignalModulationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_SIGNAL_MODULATION_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_SIGNAL_MODULATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastSignalModulationPropertyIri> for &str {
	fn eq(&self, other: &BroadcastSignalModulationPropertyIri) -> bool {
		*self == BROADCAST_SIGNAL_MODULATION_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_SIGNAL_MODULATION_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastSignalModulationPropertyIriOrLabel;
impl PartialEq<&str> for BroadcastSignalModulationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastSignalModulationPropertyIri
			|| *other == BROADCAST_SIGNAL_MODULATION_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastSignalModulationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastSignalModulationPropertyIriOrLabel) -> bool {
		*self == BroadcastSignalModulationPropertyIri
			|| *self == BROADCAST_SIGNAL_MODULATION_PROPERTY_LABEL
	}
}
