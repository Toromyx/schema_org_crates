/// <https://schema.org/PreOrderAction>
pub trait FindPreOrderActionIds {
	type IdType;
	fn find_pre_order_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPreOrderActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_pre_order_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PRE_ORDER_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PRE_ORDER_ACTION_IRI_HTTPS,
			})
		}
	}
}
