/// <https://schema.org/memberOf>
pub const MEMBER_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/memberOf";
/// <https://schema.org/memberOf>
pub const MEMBER_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/memberOf";
/// <https://schema.org/memberOf>
pub const MEMBER_OF_PROPERTY_LABEL: &str = "memberOf";
pub struct MemberOfPropertyIri;
impl PartialEq<&str> for MemberOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEMBER_OF_PROPERTY_IRI_HTTP || *other == MEMBER_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MemberOfPropertyIri> for &str {
	fn eq(&self, other: &MemberOfPropertyIri) -> bool {
		*self == MEMBER_OF_PROPERTY_IRI_HTTP || *self == MEMBER_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct MemberOfPropertyIriOrLabel;
impl PartialEq<&str> for MemberOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MemberOfPropertyIri || *other == MEMBER_OF_PROPERTY_LABEL
	}
}
impl PartialEq<MemberOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MemberOfPropertyIriOrLabel) -> bool {
		*self == MemberOfPropertyIri || *self == MEMBER_OF_PROPERTY_LABEL
	}
}
