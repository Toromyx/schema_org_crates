/// <https://schema.org/SocialEvent>
pub trait FindSocialEventIds {
	type IdType;
	fn find_social_event_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSocialEventIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_social_event_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SOCIAL_EVENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SOCIAL_EVENT_IRI_HTTPS,
			})
		}
	}
}
