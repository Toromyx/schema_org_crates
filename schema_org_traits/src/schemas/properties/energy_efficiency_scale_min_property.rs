/// <https://schema.org/energyEfficiencyScaleMin>
pub trait GetEnergyEfficiencyScaleMinProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/energyEfficiencyScaleMin>
	fn get_energy_efficiency_scale_min_property(
		&self,
		id: &Self::IdType,
	) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetEnergyEfficiencyScaleMinProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_energy_efficiency_scale_min_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
