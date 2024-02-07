/// <https://schema.org/EmailMessage>
pub trait FindEmailMessageIds {
	type IdType;
	/// <https://schema.org/EmailMessage>
	fn find_email_message_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEmailMessageIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_email_message_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EMAIL_MESSAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EMAIL_MESSAGE_IRI_HTTPS,
			})
		}
	}
}
