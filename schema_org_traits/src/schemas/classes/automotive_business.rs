/// <https://schema.org/AutomotiveBusiness>
pub trait FindAutomotiveBusinessIds {
	type IdType;
	/// <https://schema.org/AutomotiveBusiness>
	fn find_automotive_business_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAutomotiveBusinessIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_automotive_business_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::AUTOMOTIVE_BUSINESS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::AUTOMOTIVE_BUSINESS_IRI_HTTPS,
			})
		}
	}
}
