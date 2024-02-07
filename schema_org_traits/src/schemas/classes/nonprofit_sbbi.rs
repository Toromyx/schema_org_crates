/// <https://schema.org/NonprofitSBBI>
pub trait FindNonprofitSbbiIds {
	type IdType;
	/// <https://schema.org/NonprofitSBBI>
	fn find_nonprofit_sbbi_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNonprofitSbbiIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_nonprofit_sbbi_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NONPROFIT_SBBI_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NONPROFIT_SBBI_IRI_HTTPS,
			})
		}
	}
}
