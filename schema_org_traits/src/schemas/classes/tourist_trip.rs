/// <https://schema.org/TouristTrip>
pub trait FindTouristTripIds {
	type IdType;
	/// <https://schema.org/TouristTrip>
	fn find_tourist_trip_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTouristTripIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_tourist_trip_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TOURIST_TRIP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TOURIST_TRIP_IRI_HTTPS,
			})
		}
	}
}
