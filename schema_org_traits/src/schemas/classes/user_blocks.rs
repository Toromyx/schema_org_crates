/// <https://schema.org/UserBlocks>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub trait FindUserBlocksIds {
	type IdType;
	/// <https://schema.org/UserBlocks>
	#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
	fn find_user_blocks_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUserBlocksIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_user_blocks_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::USER_BLOCKS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::USER_BLOCKS_IRI_HTTPS,
			})
		}
	}
}
