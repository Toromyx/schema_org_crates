/// <https://schema.org/InviteAction>
pub trait FindInviteActionIds {
	type IdType;
	/// <https://schema.org/InviteAction>
	fn find_invite_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindInviteActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_invite_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INVITE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INVITE_ACTION_IRI_HTTPS,
			})
		}
	}
}
