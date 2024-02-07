/// <https://schema.org/StatisticalVariable>
pub trait FindStatisticalVariableIds {
	type IdType;
	/// <https://schema.org/StatisticalVariable>
	fn find_statistical_variable_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindStatisticalVariableIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_statistical_variable_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::STATISTICAL_VARIABLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::STATISTICAL_VARIABLE_IRI_HTTPS,
			})
		}
	}
}
