/// <https://schema.org/SizeSystemImperial>
pub trait FindSizeSystemImperialIds {
	type IdType;
	/// <https://schema.org/SizeSystemImperial>
	fn find_size_system_imperial_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSizeSystemImperialIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_size_system_imperial_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SIZE_SYSTEM_IMPERIAL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SIZE_SYSTEM_IMPERIAL_IRI_HTTPS,
			})
		}
	}
}
