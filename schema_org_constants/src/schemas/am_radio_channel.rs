/// <https://schema.org/AMRadioChannel>
pub const AM_RADIO_CHANNEL_IRI_HTTP: &str = "http://schema.org/AMRadioChannel";
/// <https://schema.org/AMRadioChannel>
pub const AM_RADIO_CHANNEL_IRI_HTTPS: &str = "https://schema.org/AMRadioChannel";
/// <https://schema.org/AMRadioChannel>
pub const AM_RADIO_CHANNEL_LABEL: &str = "AMRadioChannel";
pub struct AmRadioChannelIri;
impl PartialEq<&str> for AmRadioChannelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AM_RADIO_CHANNEL_IRI_HTTP || *other == AM_RADIO_CHANNEL_IRI_HTTPS
	}
}
impl PartialEq<AmRadioChannelIri> for &str {
	fn eq(&self, other: &AmRadioChannelIri) -> bool {
		*self == AM_RADIO_CHANNEL_IRI_HTTP || *self == AM_RADIO_CHANNEL_IRI_HTTPS
	}
}
pub struct AmRadioChannelIriOrLabel;
impl PartialEq<&str> for AmRadioChannelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AmRadioChannelIri || *other == AM_RADIO_CHANNEL_LABEL
	}
}
impl PartialEq<AmRadioChannelIriOrLabel> for &str {
	fn eq(&self, other: &AmRadioChannelIriOrLabel) -> bool {
		*self == AmRadioChannelIri || *self == AM_RADIO_CHANNEL_LABEL
	}
}
