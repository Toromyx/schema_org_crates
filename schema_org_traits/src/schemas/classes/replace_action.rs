/// <https://schema.org/ReplaceAction>
pub trait FindReplaceActionIds {
	type IdType;
	/// <https://schema.org/ReplaceAction>
	fn find_replace_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReplaceActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_replace_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REPLACE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REPLACE_ACTION_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReplaceActionIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_replace_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REPLACE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REPLACE_ACTION_IRI_HTTPS,
			})
		}
	}
}
