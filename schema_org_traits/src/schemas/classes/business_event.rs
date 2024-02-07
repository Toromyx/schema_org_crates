/// <https://schema.org/BusinessEvent>
pub trait FindBusinessEventIds {
	type IdType;
	/// <https://schema.org/BusinessEvent>
	fn find_business_event_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBusinessEventIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_business_event_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BUSINESS_EVENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BUSINESS_EVENT_IRI_HTTPS,
			})
		}
	}
}
