/// <https://schema.org/Airport>
pub trait FindAirportIds {
	type IdType;
	fn find_airport_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAirportIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_airport_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::AIRPORT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::AIRPORT_IRI_HTTPS,
			})
		}
	}
}
