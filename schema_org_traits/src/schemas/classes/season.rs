/// <https://schema.org/Season>
#[deprecated = "This schema is superseded by <https://schema.org/CreativeWorkSeason>."]
pub trait FindSeasonIds {
	type IdType;
	/// <https://schema.org/Season>
	#[deprecated = "This schema is superseded by <https://schema.org/CreativeWorkSeason>."]
	fn find_season_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSeasonIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_season_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SEASON_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SEASON_IRI_HTTPS,
			})
		}
	}
}
