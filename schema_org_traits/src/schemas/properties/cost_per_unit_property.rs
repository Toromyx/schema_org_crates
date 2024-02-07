/// <https://schema.org/costPerUnit>
pub trait GetCostPerUnitProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/costPerUnit>
	fn get_cost_per_unit_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetCostPerUnitProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_cost_per_unit_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::COST_PER_UNIT_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::COST_PER_UNIT_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
