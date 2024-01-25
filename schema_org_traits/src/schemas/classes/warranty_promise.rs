/// <https://schema.org/WarrantyPromise>
pub trait FindWarrantyPromiseIds {
	type IdType;
	fn find_warranty_promise_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWarrantyPromiseIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_warranty_promise_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WARRANTY_PROMISE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WARRANTY_PROMISE_IRI_HTTPS,
			})
		}
	}
}
