/// <https://schema.org/PublicSwimmingPool>
pub trait FindPublicSwimmingPoolIds {
	type IdType;
	/// <https://schema.org/PublicSwimmingPool>
	fn find_public_swimming_pool_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPublicSwimmingPoolIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_public_swimming_pool_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PUBLIC_SWIMMING_POOL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PUBLIC_SWIMMING_POOL_IRI_HTTPS,
			})
		}
	}
}
