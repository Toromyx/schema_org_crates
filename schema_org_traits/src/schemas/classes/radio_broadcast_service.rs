/// <https://schema.org/RadioBroadcastService>
pub trait FindRadioBroadcastServiceIds {
	type IdType;
	/// <https://schema.org/RadioBroadcastService>
	fn find_radio_broadcast_service_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRadioBroadcastServiceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_radio_broadcast_service_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RADIO_BROADCAST_SERVICE_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::RADIO_BROADCAST_SERVICE_IRI_HTTPS
				}
			})
		}
	}
}
