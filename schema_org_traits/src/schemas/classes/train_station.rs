/// <https://schema.org/TrainStation>
pub trait FindTrainStationIds {
	type IdType;
	fn find_train_station_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTrainStationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_train_station_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TRAIN_STATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TRAIN_STATION_IRI_HTTPS,
			})
		}
	}
}
