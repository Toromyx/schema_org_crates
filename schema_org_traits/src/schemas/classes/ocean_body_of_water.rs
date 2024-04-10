/// <https://schema.org/OceanBodyOfWater>
pub trait FindOceanBodyOfWaterIds {
	type IdType;
	/// <https://schema.org/OceanBodyOfWater>
	fn find_ocean_body_of_water_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOceanBodyOfWaterIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_ocean_body_of_water_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OCEAN_BODY_OF_WATER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OCEAN_BODY_OF_WATER_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOceanBodyOfWaterIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_ocean_body_of_water_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OCEAN_BODY_OF_WATER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OCEAN_BODY_OF_WATER_IRI_HTTPS,
			})
		}
	}
}
