/// <https://schema.org/DeliveryTimeSettings>
pub trait FindDeliveryTimeSettingsIds {
	type IdType;
	/// <https://schema.org/DeliveryTimeSettings>
	fn find_delivery_time_settings_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDeliveryTimeSettingsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_delivery_time_settings_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DELIVERY_TIME_SETTINGS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DELIVERY_TIME_SETTINGS_IRI_HTTPS,
			})
		}
	}
}
