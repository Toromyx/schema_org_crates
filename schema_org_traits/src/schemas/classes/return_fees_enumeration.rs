/// <https://schema.org/ReturnFeesEnumeration>
pub trait FindReturnFeesEnumerationIds {
	type IdType;
	/// <https://schema.org/ReturnFeesEnumeration>
	fn find_return_fees_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReturnFeesEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_return_fees_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RETURN_FEES_ENUMERATION_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::RETURN_FEES_ENUMERATION_IRI_HTTPS
				}
			})
		}
	}
}
