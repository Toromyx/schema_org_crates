/// <https://schema.org/encodesCreativeWork>
pub const ENCODES_CREATIVE_WORK_PROPERTY_IRI_HTTP: &str = "http://schema.org/encodesCreativeWork";
/// <https://schema.org/encodesCreativeWork>
pub const ENCODES_CREATIVE_WORK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/encodesCreativeWork";
/// <https://schema.org/encodesCreativeWork>
pub const ENCODES_CREATIVE_WORK_PROPERTY_LABEL: &str = "encodesCreativeWork";
pub struct EncodesCreativeWorkPropertyIri;
impl PartialEq<&str> for EncodesCreativeWorkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENCODES_CREATIVE_WORK_PROPERTY_IRI_HTTP
			|| *other == ENCODES_CREATIVE_WORK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EncodesCreativeWorkPropertyIri> for &str {
	fn eq(&self, other: &EncodesCreativeWorkPropertyIri) -> bool {
		*self == ENCODES_CREATIVE_WORK_PROPERTY_IRI_HTTP
			|| *self == ENCODES_CREATIVE_WORK_PROPERTY_IRI_HTTPS
	}
}
pub struct EncodesCreativeWorkPropertyIriOrLabel;
impl PartialEq<&str> for EncodesCreativeWorkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EncodesCreativeWorkPropertyIri || *other == ENCODES_CREATIVE_WORK_PROPERTY_LABEL
	}
}
impl PartialEq<EncodesCreativeWorkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EncodesCreativeWorkPropertyIriOrLabel) -> bool {
		*self == EncodesCreativeWorkPropertyIri || *self == ENCODES_CREATIVE_WORK_PROPERTY_LABEL
	}
}
