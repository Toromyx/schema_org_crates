/// <https://schema.org/LowLactoseDiet>
pub trait FindLowLactoseDietIds {
	type IdType;
	/// <https://schema.org/LowLactoseDiet>
	fn find_low_lactose_diet_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLowLactoseDietIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_low_lactose_diet_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LOW_LACTOSE_DIET_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LOW_LACTOSE_DIET_IRI_HTTPS,
			})
		}
	}
}
