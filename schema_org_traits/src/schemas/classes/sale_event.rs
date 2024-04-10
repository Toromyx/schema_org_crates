/// <https://schema.org/SaleEvent>
pub trait FindSaleEventIds {
	type IdType;
	/// <https://schema.org/SaleEvent>
	fn find_sale_event_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSaleEventIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_sale_event_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SALE_EVENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SALE_EVENT_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSaleEventIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_sale_event_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SALE_EVENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SALE_EVENT_IRI_HTTPS,
			})
		}
	}
}
