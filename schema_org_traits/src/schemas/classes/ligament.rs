/// <https://schema.org/Ligament>
pub trait FindLigamentIds {
	type IdType;
	fn find_ligament_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLigamentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_ligament_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LIGAMENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LIGAMENT_IRI_HTTPS,
			})
		}
	}
}
