/// <https://schema.org/DefinitiveLegalValue>
pub trait FindDefinitiveLegalValueIds {
	type IdType;
	fn find_definitive_legal_value_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDefinitiveLegalValueIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_definitive_legal_value_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DEFINITIVE_LEGAL_VALUE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DEFINITIVE_LEGAL_VALUE_IRI_HTTPS,
			})
		}
	}
}
