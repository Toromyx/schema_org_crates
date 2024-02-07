/// <https://schema.org/MotorcycleDealer>
pub trait FindMotorcycleDealerIds {
	type IdType;
	/// <https://schema.org/MotorcycleDealer>
	fn find_motorcycle_dealer_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMotorcycleDealerIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_motorcycle_dealer_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MOTORCYCLE_DEALER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MOTORCYCLE_DEALER_IRI_HTTPS,
			})
		}
	}
}
