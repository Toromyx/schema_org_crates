/// <https://schema.org/PublicHealth>
pub trait FindPublicHealthIds {
	type IdType;
	/// <https://schema.org/PublicHealth>
	fn find_public_health_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPublicHealthIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_public_health_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PUBLIC_HEALTH_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PUBLIC_HEALTH_IRI_HTTPS,
			})
		}
	}
}
