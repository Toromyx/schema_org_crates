/// <https://schema.org/ParkingFacility>
pub trait FindParkingFacilityIds {
	type IdType;
	/// <https://schema.org/ParkingFacility>
	fn find_parking_facility_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindParkingFacilityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_parking_facility_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PARKING_FACILITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PARKING_FACILITY_IRI_HTTPS,
			})
		}
	}
}
