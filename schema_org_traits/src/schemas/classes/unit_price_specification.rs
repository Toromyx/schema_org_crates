/// <https://schema.org/UnitPriceSpecification>
pub trait FindUnitPriceSpecificationIds {
	type IdType;
	fn find_unit_price_specification_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUnitPriceSpecificationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_unit_price_specification_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::UNIT_PRICE_SPECIFICATION_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::UNIT_PRICE_SPECIFICATION_IRI_HTTPS
				}
			})
		}
	}
}
