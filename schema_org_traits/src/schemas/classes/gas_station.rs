/// <https://schema.org/GasStation>
pub trait FindGasStationIds {
	type IdType;
	/// <https://schema.org/GasStation>
	fn find_gas_station_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGasStationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_gas_station_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GAS_STATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GAS_STATION_IRI_HTTPS,
			})
		}
	}
}
