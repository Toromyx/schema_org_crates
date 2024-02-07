/// <https://schema.org/XRay>
pub trait FindXRayIds {
	type IdType;
	/// <https://schema.org/XRay>
	fn find_x_ray_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindXRayIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_x_ray_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::X_RAY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::X_RAY_IRI_HTTPS,
			})
		}
	}
}
