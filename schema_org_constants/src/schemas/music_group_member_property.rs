/// <https://schema.org/musicGroupMember>
#[deprecated = "This schema is superseded by <https://schema.org/member>."]
pub const MUSIC_GROUP_MEMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/musicGroupMember";
/// <https://schema.org/musicGroupMember>
#[deprecated = "This schema is superseded by <https://schema.org/member>."]
pub const MUSIC_GROUP_MEMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/musicGroupMember";
/// <https://schema.org/musicGroupMember>
#[deprecated = "This schema is superseded by <https://schema.org/member>."]
pub const MUSIC_GROUP_MEMBER_PROPERTY_LABEL: &str = "musicGroupMember";
pub struct MusicGroupMemberPropertyIri;
impl PartialEq<&str> for MusicGroupMemberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_GROUP_MEMBER_PROPERTY_IRI_HTTP
			|| *other == MUSIC_GROUP_MEMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MusicGroupMemberPropertyIri> for &str {
	fn eq(&self, other: &MusicGroupMemberPropertyIri) -> bool {
		*self == MUSIC_GROUP_MEMBER_PROPERTY_IRI_HTTP
			|| *self == MUSIC_GROUP_MEMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct MusicGroupMemberPropertyIriOrLabel;
impl PartialEq<&str> for MusicGroupMemberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicGroupMemberPropertyIri || *other == MUSIC_GROUP_MEMBER_PROPERTY_LABEL
	}
}
impl PartialEq<MusicGroupMemberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MusicGroupMemberPropertyIriOrLabel) -> bool {
		*self == MusicGroupMemberPropertyIri || *self == MUSIC_GROUP_MEMBER_PROPERTY_LABEL
	}
}
