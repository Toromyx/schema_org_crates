/// <https://schema.org/BroadcastService>
pub trait FindBroadcastServiceIds {
	type IdType;
	fn find_broadcast_service_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBroadcastServiceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_broadcast_service_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BROADCAST_SERVICE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BROADCAST_SERVICE_IRI_HTTPS,
			})
		}
	}
}
