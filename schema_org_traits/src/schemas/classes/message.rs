/// <https://schema.org/Message>
pub trait FindMessageIds {
	type IdType;
	fn find_message_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMessageIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_message_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MESSAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MESSAGE_IRI_HTTPS,
			})
		}
	}
}
