/// <https://schema.org/EndorseAction>
pub trait FindEndorseActionIds {
	type IdType;
	fn find_endorse_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEndorseActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_endorse_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ENDORSE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ENDORSE_ACTION_IRI_HTTPS,
			})
		}
	}
}
