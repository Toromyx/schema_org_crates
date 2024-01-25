/// <https://schema.org/CommunityHealth>
pub const COMMUNITY_HEALTH_IRI_HTTP: &str = "http://schema.org/CommunityHealth";
/// <https://schema.org/CommunityHealth>
pub const COMMUNITY_HEALTH_IRI_HTTPS: &str = "https://schema.org/CommunityHealth";
/// <https://schema.org/CommunityHealth>
pub const COMMUNITY_HEALTH_LABEL: &str = "CommunityHealth";
pub struct CommunityHealthIri;
impl PartialEq<&str> for CommunityHealthIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMUNITY_HEALTH_IRI_HTTP || *other == COMMUNITY_HEALTH_IRI_HTTPS
	}
}
impl PartialEq<CommunityHealthIri> for &str {
	fn eq(&self, other: &CommunityHealthIri) -> bool {
		*self == COMMUNITY_HEALTH_IRI_HTTP || *self == COMMUNITY_HEALTH_IRI_HTTPS
	}
}
pub struct CommunityHealthIriOrLabel;
impl PartialEq<&str> for CommunityHealthIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommunityHealthIri || *other == COMMUNITY_HEALTH_LABEL
	}
}
impl PartialEq<CommunityHealthIriOrLabel> for &str {
	fn eq(&self, other: &CommunityHealthIriOrLabel) -> bool {
		*self == CommunityHealthIri || *self == COMMUNITY_HEALTH_LABEL
	}
}
