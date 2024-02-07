/// <https://schema.org/Longitudinal>
pub trait FindLongitudinalIds {
	type IdType;
	/// <https://schema.org/Longitudinal>
	fn find_longitudinal_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLongitudinalIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_longitudinal_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LONGITUDINAL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LONGITUDINAL_IRI_HTTPS,
			})
		}
	}
}
