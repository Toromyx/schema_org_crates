/// <https://schema.org/mentions>
pub const MENTIONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/mentions";
/// <https://schema.org/mentions>
pub const MENTIONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mentions";
/// <https://schema.org/mentions>
pub const MENTIONS_PROPERTY_LABEL: &str = "mentions";
pub struct MentionsPropertyIri;
impl PartialEq<&str> for MentionsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MENTIONS_PROPERTY_IRI_HTTP || *other == MENTIONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MentionsPropertyIri> for &str {
	fn eq(&self, other: &MentionsPropertyIri) -> bool {
		*self == MENTIONS_PROPERTY_IRI_HTTP || *self == MENTIONS_PROPERTY_IRI_HTTPS
	}
}
pub struct MentionsPropertyIriOrLabel;
impl PartialEq<&str> for MentionsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MentionsPropertyIri || *other == MENTIONS_PROPERTY_LABEL
	}
}
impl PartialEq<MentionsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MentionsPropertyIriOrLabel) -> bool {
		*self == MentionsPropertyIri || *self == MENTIONS_PROPERTY_LABEL
	}
}
