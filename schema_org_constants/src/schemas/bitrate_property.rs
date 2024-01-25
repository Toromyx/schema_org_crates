/// <https://schema.org/bitrate>
pub const BITRATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/bitrate";
/// <https://schema.org/bitrate>
pub const BITRATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bitrate";
/// <https://schema.org/bitrate>
pub const BITRATE_PROPERTY_LABEL: &str = "bitrate";
pub struct BitratePropertyIri;
impl PartialEq<&str> for BitratePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BITRATE_PROPERTY_IRI_HTTP || *other == BITRATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BitratePropertyIri> for &str {
	fn eq(&self, other: &BitratePropertyIri) -> bool {
		*self == BITRATE_PROPERTY_IRI_HTTP || *self == BITRATE_PROPERTY_IRI_HTTPS
	}
}
pub struct BitratePropertyIriOrLabel;
impl PartialEq<&str> for BitratePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BitratePropertyIri || *other == BITRATE_PROPERTY_LABEL
	}
}
impl PartialEq<BitratePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BitratePropertyIriOrLabel) -> bool {
		*self == BitratePropertyIri || *self == BITRATE_PROPERTY_LABEL
	}
}
