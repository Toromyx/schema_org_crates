/// <https://schema.org/UserTweets>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub trait FindUserTweetsIds {
	type IdType;
	#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
	fn find_user_tweets_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUserTweetsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_user_tweets_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::USER_TWEETS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::USER_TWEETS_IRI_HTTPS,
			})
		}
	}
}
