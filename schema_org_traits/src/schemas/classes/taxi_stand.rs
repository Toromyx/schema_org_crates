/// <https://schema.org/TaxiStand>
pub trait FindTaxiStandIds {
	type IdType;
	fn find_taxi_stand_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTaxiStandIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_taxi_stand_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TAXI_STAND_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TAXI_STAND_IRI_HTTPS,
			})
		}
	}
}
