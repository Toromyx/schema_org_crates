/// <https://schema.org/IndividualProduct>
pub trait FindIndividualProductIds {
	type IdType;
	fn find_individual_product_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindIndividualProductIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_individual_product_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INDIVIDUAL_PRODUCT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INDIVIDUAL_PRODUCT_IRI_HTTPS,
			})
		}
	}
}
