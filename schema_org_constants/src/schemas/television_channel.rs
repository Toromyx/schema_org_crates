/// <https://schema.org/TelevisionChannel>
pub const TELEVISION_CHANNEL_IRI_HTTP: &str = "http://schema.org/TelevisionChannel";
/// <https://schema.org/TelevisionChannel>
pub const TELEVISION_CHANNEL_IRI_HTTPS: &str = "https://schema.org/TelevisionChannel";
/// <https://schema.org/TelevisionChannel>
pub const TELEVISION_CHANNEL_LABEL: &str = "TelevisionChannel";
pub struct TelevisionChannelIri;
impl PartialEq<&str> for TelevisionChannelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TELEVISION_CHANNEL_IRI_HTTP || *other == TELEVISION_CHANNEL_IRI_HTTPS
	}
}
impl PartialEq<TelevisionChannelIri> for &str {
	fn eq(&self, other: &TelevisionChannelIri) -> bool {
		*self == TELEVISION_CHANNEL_IRI_HTTP || *self == TELEVISION_CHANNEL_IRI_HTTPS
	}
}
pub struct TelevisionChannelIriOrLabel;
impl PartialEq<&str> for TelevisionChannelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TelevisionChannelIri || *other == TELEVISION_CHANNEL_LABEL
	}
}
impl PartialEq<TelevisionChannelIriOrLabel> for &str {
	fn eq(&self, other: &TelevisionChannelIriOrLabel) -> bool {
		*self == TelevisionChannelIri || *self == TELEVISION_CHANNEL_LABEL
	}
}
