/// <https://schema.org/NailSalon>
pub trait FindNailSalonIds {
	type IdType;
	/// <https://schema.org/NailSalon>
	fn find_nail_salon_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNailSalonIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_nail_salon_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NAIL_SALON_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NAIL_SALON_IRI_HTTPS,
			})
		}
	}
}
