/// <https://schema.org/FMRadioChannel>
pub const FM_RADIO_CHANNEL_IRI_HTTP: &str = "http://schema.org/FMRadioChannel";
/// <https://schema.org/FMRadioChannel>
pub const FM_RADIO_CHANNEL_IRI_HTTPS: &str = "https://schema.org/FMRadioChannel";
/// <https://schema.org/FMRadioChannel>
pub const FM_RADIO_CHANNEL_LABEL: &str = "FMRadioChannel";
pub struct FmRadioChannelIri;
impl PartialEq<&str> for FmRadioChannelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FM_RADIO_CHANNEL_IRI_HTTP || *other == FM_RADIO_CHANNEL_IRI_HTTPS
	}
}
impl PartialEq<FmRadioChannelIri> for &str {
	fn eq(&self, other: &FmRadioChannelIri) -> bool {
		*self == FM_RADIO_CHANNEL_IRI_HTTP || *self == FM_RADIO_CHANNEL_IRI_HTTPS
	}
}
pub struct FmRadioChannelIriOrLabel;
impl PartialEq<&str> for FmRadioChannelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FmRadioChannelIri || *other == FM_RADIO_CHANNEL_LABEL
	}
}
impl PartialEq<FmRadioChannelIriOrLabel> for &str {
	fn eq(&self, other: &FmRadioChannelIriOrLabel) -> bool {
		*self == FmRadioChannelIri || *self == FM_RADIO_CHANNEL_LABEL
	}
}
