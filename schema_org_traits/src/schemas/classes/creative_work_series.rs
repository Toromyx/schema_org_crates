/// <https://schema.org/CreativeWorkSeries>
pub trait FindCreativeWorkSeriesIds {
	type IdType;
	/// <https://schema.org/CreativeWorkSeries>
	fn find_creative_work_series_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCreativeWorkSeriesIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_creative_work_series_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CREATIVE_WORK_SERIES_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CREATIVE_WORK_SERIES_IRI_HTTPS,
			})
		}
	}
}
