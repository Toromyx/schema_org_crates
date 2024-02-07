/// <https://schema.org/Flight>
pub trait FindFlightIds {
	type IdType;
	/// <https://schema.org/Flight>
	fn find_flight_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFlightIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_flight_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FLIGHT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FLIGHT_IRI_HTTPS,
			})
		}
	}
}
