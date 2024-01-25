/// <https://schema.org/CompleteDataFeed>
pub trait FindCompleteDataFeedIds {
	type IdType;
	fn find_complete_data_feed_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCompleteDataFeedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_complete_data_feed_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::COMPLETE_DATA_FEED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::COMPLETE_DATA_FEED_IRI_HTTPS,
			})
		}
	}
}
