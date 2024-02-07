/// <https://schema.org/UserReview>
pub trait FindUserReviewIds {
	type IdType;
	/// <https://schema.org/UserReview>
	fn find_user_review_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUserReviewIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_user_review_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::USER_REVIEW_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::USER_REVIEW_IRI_HTTPS,
			})
		}
	}
}
