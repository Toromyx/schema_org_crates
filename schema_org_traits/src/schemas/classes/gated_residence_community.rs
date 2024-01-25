/// <https://schema.org/GatedResidenceCommunity>
pub trait FindGatedResidenceCommunityIds {
	type IdType;
	fn find_gated_residence_community_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGatedResidenceCommunityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_gated_residence_community_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::GATED_RESIDENCE_COMMUNITY_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::GATED_RESIDENCE_COMMUNITY_IRI_HTTPS
				}
			})
		}
	}
}
