/// <https://schema.org/PriceTypeEnumeration>
pub trait FindPriceTypeEnumerationIds {
	type IdType;
	/// <https://schema.org/PriceTypeEnumeration>
	fn find_price_type_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPriceTypeEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_price_type_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PRICE_TYPE_ENUMERATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PRICE_TYPE_ENUMERATION_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPriceTypeEnumerationIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_price_type_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PRICE_TYPE_ENUMERATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PRICE_TYPE_ENUMERATION_IRI_HTTPS,
			})
		}
	}
}
