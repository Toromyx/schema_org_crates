/// <https://schema.org/Sculpture>
pub trait FindSculptureIds {
	type IdType;
	/// <https://schema.org/Sculpture>
	fn find_sculpture_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSculptureIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_sculpture_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SCULPTURE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SCULPTURE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSculptureIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_sculpture_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SCULPTURE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SCULPTURE_IRI_HTTPS,
			})
		}
	}
}
