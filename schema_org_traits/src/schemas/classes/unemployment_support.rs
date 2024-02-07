/// <https://schema.org/UnemploymentSupport>
pub trait FindUnemploymentSupportIds {
	type IdType;
	/// <https://schema.org/UnemploymentSupport>
	fn find_unemployment_support_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUnemploymentSupportIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_unemployment_support_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::UNEMPLOYMENT_SUPPORT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::UNEMPLOYMENT_SUPPORT_IRI_HTTPS,
			})
		}
	}
}
