/// <https://schema.org/AgreeAction>
pub trait FindAgreeActionIds {
	type IdType;
	/// <https://schema.org/AgreeAction>
	fn find_agree_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAgreeActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_agree_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::AGREE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::AGREE_ACTION_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAgreeActionIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_agree_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::AGREE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::AGREE_ACTION_IRI_HTTPS,
			})
		}
	}
}
