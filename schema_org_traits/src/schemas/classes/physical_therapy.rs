/// <https://schema.org/PhysicalTherapy>
pub trait FindPhysicalTherapyIds {
	type IdType;
	/// <https://schema.org/PhysicalTherapy>
	fn find_physical_therapy_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPhysicalTherapyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_physical_therapy_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PHYSICAL_THERAPY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PHYSICAL_THERAPY_IRI_HTTPS,
			})
		}
	}
}
