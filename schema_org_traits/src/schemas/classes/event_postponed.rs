/// <https://schema.org/EventPostponed>
pub trait FindEventPostponedIds {
	type IdType;
	/// <https://schema.org/EventPostponed>
	fn find_event_postponed_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEventPostponedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_event_postponed_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EVENT_POSTPONED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EVENT_POSTPONED_IRI_HTTPS,
			})
		}
	}
}
