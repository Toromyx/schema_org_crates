/// <https://schema.org/ShippingRateSettings>
pub trait FindShippingRateSettingsIds {
	type IdType;
	/// <https://schema.org/ShippingRateSettings>
	fn find_shipping_rate_settings_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindShippingRateSettingsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_shipping_rate_settings_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SHIPPING_RATE_SETTINGS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SHIPPING_RATE_SETTINGS_IRI_HTTPS,
			})
		}
	}
}
