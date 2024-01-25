/// <https://schema.org/originalMediaLink>
pub const ORIGINAL_MEDIA_LINK_PROPERTY_IRI_HTTP: &str = "http://schema.org/originalMediaLink";
/// <https://schema.org/originalMediaLink>
pub const ORIGINAL_MEDIA_LINK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/originalMediaLink";
/// <https://schema.org/originalMediaLink>
pub const ORIGINAL_MEDIA_LINK_PROPERTY_LABEL: &str = "originalMediaLink";
pub struct OriginalMediaLinkPropertyIri;
impl PartialEq<&str> for OriginalMediaLinkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORIGINAL_MEDIA_LINK_PROPERTY_IRI_HTTP
			|| *other == ORIGINAL_MEDIA_LINK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OriginalMediaLinkPropertyIri> for &str {
	fn eq(&self, other: &OriginalMediaLinkPropertyIri) -> bool {
		*self == ORIGINAL_MEDIA_LINK_PROPERTY_IRI_HTTP
			|| *self == ORIGINAL_MEDIA_LINK_PROPERTY_IRI_HTTPS
	}
}
pub struct OriginalMediaLinkPropertyIriOrLabel;
impl PartialEq<&str> for OriginalMediaLinkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OriginalMediaLinkPropertyIri || *other == ORIGINAL_MEDIA_LINK_PROPERTY_LABEL
	}
}
impl PartialEq<OriginalMediaLinkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OriginalMediaLinkPropertyIriOrLabel) -> bool {
		*self == OriginalMediaLinkPropertyIri || *self == ORIGINAL_MEDIA_LINK_PROPERTY_LABEL
	}
}
