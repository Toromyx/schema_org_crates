/// <https://schema.org/CreativeWorkSeason>
pub trait FindCreativeWorkSeasonIds {
	type IdType;
	/// <https://schema.org/CreativeWorkSeason>
	fn find_creative_work_season_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCreativeWorkSeasonIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_creative_work_season_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CREATIVE_WORK_SEASON_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CREATIVE_WORK_SEASON_IRI_HTTPS,
			})
		}
	}
}
