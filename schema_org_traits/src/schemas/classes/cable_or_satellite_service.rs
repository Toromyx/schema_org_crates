/// <https://schema.org/CableOrSatelliteService>
pub trait FindCableOrSatelliteServiceIds {
	type IdType;
	/// <https://schema.org/CableOrSatelliteService>
	fn find_cable_or_satellite_service_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCableOrSatelliteServiceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_cable_or_satellite_service_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::CABLE_OR_SATELLITE_SERVICE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::CABLE_OR_SATELLITE_SERVICE_IRI_HTTPS
				}
			})
		}
	}
}
