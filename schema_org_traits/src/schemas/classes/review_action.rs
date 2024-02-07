/// <https://schema.org/ReviewAction>
pub trait FindReviewActionIds {
	type IdType;
	/// <https://schema.org/ReviewAction>
	fn find_review_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReviewActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_review_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REVIEW_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REVIEW_ACTION_IRI_HTTPS,
			})
		}
	}
}
