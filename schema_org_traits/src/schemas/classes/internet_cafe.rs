/// <https://schema.org/InternetCafe>
pub trait FindInternetCafeIds {
	type IdType;
	/// <https://schema.org/InternetCafe>
	fn find_internet_cafe_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindInternetCafeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_internet_cafe_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INTERNET_CAFE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INTERNET_CAFE_IRI_HTTPS,
			})
		}
	}
}
