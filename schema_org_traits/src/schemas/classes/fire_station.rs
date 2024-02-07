/// <https://schema.org/FireStation>
pub trait FindFireStationIds {
	type IdType;
	/// <https://schema.org/FireStation>
	fn find_fire_station_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFireStationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_fire_station_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FIRE_STATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FIRE_STATION_IRI_HTTPS,
			})
		}
	}
}
