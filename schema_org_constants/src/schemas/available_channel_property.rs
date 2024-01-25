/// <https://schema.org/availableChannel>
pub const AVAILABLE_CHANNEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableChannel";
/// <https://schema.org/availableChannel>
pub const AVAILABLE_CHANNEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableChannel";
/// <https://schema.org/availableChannel>
pub const AVAILABLE_CHANNEL_PROPERTY_LABEL: &str = "availableChannel";
pub struct AvailableChannelPropertyIri;
impl PartialEq<&str> for AvailableChannelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_CHANNEL_PROPERTY_IRI_HTTP
			|| *other == AVAILABLE_CHANNEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableChannelPropertyIri> for &str {
	fn eq(&self, other: &AvailableChannelPropertyIri) -> bool {
		*self == AVAILABLE_CHANNEL_PROPERTY_IRI_HTTP
			|| *self == AVAILABLE_CHANNEL_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableChannelPropertyIriOrLabel;
impl PartialEq<&str> for AvailableChannelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableChannelPropertyIri || *other == AVAILABLE_CHANNEL_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableChannelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableChannelPropertyIriOrLabel) -> bool {
		*self == AvailableChannelPropertyIri || *self == AVAILABLE_CHANNEL_PROPERTY_LABEL
	}
}
