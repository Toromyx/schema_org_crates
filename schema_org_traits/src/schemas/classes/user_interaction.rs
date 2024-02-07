/// <https://schema.org/UserInteraction>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub trait FindUserInteractionIds {
	type IdType;
	/// <https://schema.org/UserInteraction>
	#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
	fn find_user_interaction_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUserInteractionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_user_interaction_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::USER_INTERACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::USER_INTERACTION_IRI_HTTPS,
			})
		}
	}
}
