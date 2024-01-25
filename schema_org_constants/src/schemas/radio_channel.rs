/// <https://schema.org/RadioChannel>
pub const RADIO_CHANNEL_IRI_HTTP: &str = "http://schema.org/RadioChannel";
/// <https://schema.org/RadioChannel>
pub const RADIO_CHANNEL_IRI_HTTPS: &str = "https://schema.org/RadioChannel";
/// <https://schema.org/RadioChannel>
pub const RADIO_CHANNEL_LABEL: &str = "RadioChannel";
pub struct RadioChannelIri;
impl PartialEq<&str> for RadioChannelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIO_CHANNEL_IRI_HTTP || *other == RADIO_CHANNEL_IRI_HTTPS
	}
}
impl PartialEq<RadioChannelIri> for &str {
	fn eq(&self, other: &RadioChannelIri) -> bool {
		*self == RADIO_CHANNEL_IRI_HTTP || *self == RADIO_CHANNEL_IRI_HTTPS
	}
}
pub struct RadioChannelIriOrLabel;
impl PartialEq<&str> for RadioChannelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadioChannelIri || *other == RADIO_CHANNEL_LABEL
	}
}
impl PartialEq<RadioChannelIriOrLabel> for &str {
	fn eq(&self, other: &RadioChannelIriOrLabel) -> bool {
		*self == RadioChannelIri || *self == RADIO_CHANNEL_LABEL
	}
}
