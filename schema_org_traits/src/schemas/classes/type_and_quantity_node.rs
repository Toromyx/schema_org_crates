/// <https://schema.org/TypeAndQuantityNode>
pub trait FindTypeAndQuantityNodeIds {
	type IdType;
	/// <https://schema.org/TypeAndQuantityNode>
	fn find_type_and_quantity_node_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTypeAndQuantityNodeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_type_and_quantity_node_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TYPE_AND_QUANTITY_NODE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TYPE_AND_QUANTITY_NODE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTypeAndQuantityNodeIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_type_and_quantity_node_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TYPE_AND_QUANTITY_NODE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TYPE_AND_QUANTITY_NODE_IRI_HTTPS,
			})
		}
	}
}
