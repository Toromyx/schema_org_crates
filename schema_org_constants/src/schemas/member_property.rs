/// <https://schema.org/member>
pub const MEMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/member";
/// <https://schema.org/member>
pub const MEMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/member";
/// <https://schema.org/member>
pub const MEMBER_PROPERTY_LABEL: &str = "member";
pub struct MemberPropertyIri;
impl PartialEq<&str> for MemberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEMBER_PROPERTY_IRI_HTTP || *other == MEMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MemberPropertyIri> for &str {
	fn eq(&self, other: &MemberPropertyIri) -> bool {
		*self == MEMBER_PROPERTY_IRI_HTTP || *self == MEMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct MemberPropertyIriOrLabel;
impl PartialEq<&str> for MemberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MemberPropertyIri || *other == MEMBER_PROPERTY_LABEL
	}
}
impl PartialEq<MemberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MemberPropertyIriOrLabel) -> bool {
		*self == MemberPropertyIri || *self == MEMBER_PROPERTY_LABEL
	}
}
