/// <https://schema.org/OrderReturned>
pub trait FindOrderReturnedIds {
	type IdType;
	/// <https://schema.org/OrderReturned>
	fn find_order_returned_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOrderReturnedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_order_returned_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ORDER_RETURNED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ORDER_RETURNED_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOrderReturnedIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_order_returned_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ORDER_RETURNED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ORDER_RETURNED_IRI_HTTPS,
			})
		}
	}
}
