/// <https://schema.org/Taxi>
#[deprecated = "This schema is superseded by <https://schema.org/TaxiService>."]
pub trait FindTaxiIds {
	type IdType;
	/// <https://schema.org/Taxi>
	#[deprecated = "This schema is superseded by <https://schema.org/TaxiService>."]
	fn find_taxi_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTaxiIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_taxi_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TAXI_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TAXI_IRI_HTTPS,
			})
		}
	}
}
