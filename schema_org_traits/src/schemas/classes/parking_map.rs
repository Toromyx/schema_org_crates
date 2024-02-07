/// <https://schema.org/ParkingMap>
pub trait FindParkingMapIds {
	type IdType;
	/// <https://schema.org/ParkingMap>
	fn find_parking_map_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindParkingMapIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_parking_map_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PARKING_MAP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PARKING_MAP_IRI_HTTPS,
			})
		}
	}
}
