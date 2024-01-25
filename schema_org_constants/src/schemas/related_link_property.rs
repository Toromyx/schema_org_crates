/// <https://schema.org/relatedLink>
pub const RELATED_LINK_PROPERTY_IRI_HTTP: &str = "http://schema.org/relatedLink";
/// <https://schema.org/relatedLink>
pub const RELATED_LINK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relatedLink";
/// <https://schema.org/relatedLink>
pub const RELATED_LINK_PROPERTY_LABEL: &str = "relatedLink";
pub struct RelatedLinkPropertyIri;
impl PartialEq<&str> for RelatedLinkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELATED_LINK_PROPERTY_IRI_HTTP || *other == RELATED_LINK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelatedLinkPropertyIri> for &str {
	fn eq(&self, other: &RelatedLinkPropertyIri) -> bool {
		*self == RELATED_LINK_PROPERTY_IRI_HTTP || *self == RELATED_LINK_PROPERTY_IRI_HTTPS
	}
}
pub struct RelatedLinkPropertyIriOrLabel;
impl PartialEq<&str> for RelatedLinkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelatedLinkPropertyIri || *other == RELATED_LINK_PROPERTY_LABEL
	}
}
impl PartialEq<RelatedLinkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelatedLinkPropertyIriOrLabel) -> bool {
		*self == RelatedLinkPropertyIri || *self == RELATED_LINK_PROPERTY_LABEL
	}
}
