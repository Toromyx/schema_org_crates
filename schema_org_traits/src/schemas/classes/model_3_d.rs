/// <https://schema.org/3DModel>
pub trait FindModel3DIds {
	type IdType;
	/// <https://schema.org/3DModel>
	fn find_model_3_d_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindModel3DIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_model_3_d_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MODEL_3_D_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MODEL_3_D_IRI_HTTPS,
			})
		}
	}
}
