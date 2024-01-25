/// <https://schema.org/DaySpa>
pub trait FindDaySpaIds {
	type IdType;
	fn find_day_spa_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDaySpaIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_day_spa_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DAY_SPA_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DAY_SPA_IRI_HTTPS,
			})
		}
	}
}
