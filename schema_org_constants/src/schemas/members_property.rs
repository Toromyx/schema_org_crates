/// <https://schema.org/members>
#[deprecated = "This schema is superseded by <https://schema.org/member>."]
pub const MEMBERS_PROPERTY_IRI_HTTP: &str = "http://schema.org/members";
/// <https://schema.org/members>
#[deprecated = "This schema is superseded by <https://schema.org/member>."]
pub const MEMBERS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/members";
/// <https://schema.org/members>
#[deprecated = "This schema is superseded by <https://schema.org/member>."]
pub const MEMBERS_PROPERTY_LABEL: &str = "members";
pub struct MembersPropertyIri;
impl PartialEq<&str> for MembersPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEMBERS_PROPERTY_IRI_HTTP || *other == MEMBERS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MembersPropertyIri> for &str {
	fn eq(&self, other: &MembersPropertyIri) -> bool {
		*self == MEMBERS_PROPERTY_IRI_HTTP || *self == MEMBERS_PROPERTY_IRI_HTTPS
	}
}
pub struct MembersPropertyIriOrLabel;
impl PartialEq<&str> for MembersPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MembersPropertyIri || *other == MEMBERS_PROPERTY_LABEL
	}
}
impl PartialEq<MembersPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MembersPropertyIriOrLabel) -> bool {
		*self == MembersPropertyIri || *self == MEMBERS_PROPERTY_LABEL
	}
}
