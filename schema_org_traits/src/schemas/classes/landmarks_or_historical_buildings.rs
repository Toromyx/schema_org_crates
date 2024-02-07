/// <https://schema.org/LandmarksOrHistoricalBuildings>
pub trait FindLandmarksOrHistoricalBuildingsIds {
	type IdType;
	/// <https://schema.org/LandmarksOrHistoricalBuildings>
	fn find_landmarks_or_historical_buildings_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLandmarksOrHistoricalBuildingsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_landmarks_or_historical_buildings_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::LANDMARKS_OR_HISTORICAL_BUILDINGS_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::LANDMARKS_OR_HISTORICAL_BUILDINGS_IRI_HTTPS
				}
			})
		}
	}
}
