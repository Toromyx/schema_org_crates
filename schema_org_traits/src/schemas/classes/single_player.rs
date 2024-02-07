/// <https://schema.org/SinglePlayer>
pub trait FindSinglePlayerIds {
	type IdType;
	/// <https://schema.org/SinglePlayer>
	fn find_single_player_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSinglePlayerIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_single_player_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SINGLE_PLAYER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SINGLE_PLAYER_IRI_HTTPS,
			})
		}
	}
}
