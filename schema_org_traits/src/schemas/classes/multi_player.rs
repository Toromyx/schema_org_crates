/// <https://schema.org/MultiPlayer>
pub trait FindMultiPlayerIds {
	type IdType;
	/// <https://schema.org/MultiPlayer>
	fn find_multi_player_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMultiPlayerIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_multi_player_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MULTI_PLAYER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MULTI_PLAYER_IRI_HTTPS,
			})
		}
	}
}
