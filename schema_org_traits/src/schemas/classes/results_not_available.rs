/// <https://schema.org/ResultsNotAvailable>
pub trait FindResultsNotAvailableIds {
	type IdType;
	/// <https://schema.org/ResultsNotAvailable>
	fn find_results_not_available_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindResultsNotAvailableIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_results_not_available_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESULTS_NOT_AVAILABLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESULTS_NOT_AVAILABLE_IRI_HTTPS,
			})
		}
	}
}
