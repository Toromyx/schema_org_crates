/// <https://schema.org/PriceComponentTypeEnumeration>
pub trait FindPriceComponentTypeEnumerationIds {
	type IdType;
	/// <https://schema.org/PriceComponentTypeEnumeration>
	fn find_price_component_type_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPriceComponentTypeEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_price_component_type_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PRICE_COMPONENT_TYPE_ENUMERATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PRICE_COMPONENT_TYPE_ENUMERATION_IRI_HTTPS
				}
			})
		}
	}
}
