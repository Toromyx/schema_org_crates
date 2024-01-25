/// <https://schema.org/CaseSeries>
pub trait FindCaseSeriesIds {
	type IdType;
	fn find_case_series_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCaseSeriesIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_case_series_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CASE_SERIES_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CASE_SERIES_IRI_HTTPS,
			})
		}
	}
}
