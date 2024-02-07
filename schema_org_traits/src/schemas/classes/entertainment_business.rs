/// <https://schema.org/EntertainmentBusiness>
pub trait FindEntertainmentBusinessIds {
	type IdType;
	/// <https://schema.org/EntertainmentBusiness>
	fn find_entertainment_business_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEntertainmentBusinessIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_entertainment_business_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ENTERTAINMENT_BUSINESS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ENTERTAINMENT_BUSINESS_IRI_HTTPS,
			})
		}
	}
}
