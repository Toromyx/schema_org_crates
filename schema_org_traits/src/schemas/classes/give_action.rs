/// <https://schema.org/GiveAction>
pub trait FindGiveActionIds {
	type IdType;
	/// <https://schema.org/GiveAction>
	fn find_give_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGiveActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_give_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GIVE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GIVE_ACTION_IRI_HTTPS,
			})
		}
	}
}
