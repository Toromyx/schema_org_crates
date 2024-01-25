/// <https://schema.org/GatedResidenceCommunity>
pub const GATED_RESIDENCE_COMMUNITY_IRI_HTTP: &str = "http://schema.org/GatedResidenceCommunity";
/// <https://schema.org/GatedResidenceCommunity>
pub const GATED_RESIDENCE_COMMUNITY_IRI_HTTPS: &str = "https://schema.org/GatedResidenceCommunity";
/// <https://schema.org/GatedResidenceCommunity>
pub const GATED_RESIDENCE_COMMUNITY_LABEL: &str = "GatedResidenceCommunity";
pub struct GatedResidenceCommunityIri;
impl PartialEq<&str> for GatedResidenceCommunityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GATED_RESIDENCE_COMMUNITY_IRI_HTTP
			|| *other == GATED_RESIDENCE_COMMUNITY_IRI_HTTPS
	}
}
impl PartialEq<GatedResidenceCommunityIri> for &str {
	fn eq(&self, other: &GatedResidenceCommunityIri) -> bool {
		*self == GATED_RESIDENCE_COMMUNITY_IRI_HTTP || *self == GATED_RESIDENCE_COMMUNITY_IRI_HTTPS
	}
}
pub struct GatedResidenceCommunityIriOrLabel;
impl PartialEq<&str> for GatedResidenceCommunityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GatedResidenceCommunityIri || *other == GATED_RESIDENCE_COMMUNITY_LABEL
	}
}
impl PartialEq<GatedResidenceCommunityIriOrLabel> for &str {
	fn eq(&self, other: &GatedResidenceCommunityIriOrLabel) -> bool {
		*self == GatedResidenceCommunityIri || *self == GATED_RESIDENCE_COMMUNITY_LABEL
	}
}
