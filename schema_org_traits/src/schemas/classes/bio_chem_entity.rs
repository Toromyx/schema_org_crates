/// <https://schema.org/BioChemEntity>
pub trait FindBioChemEntityIds {
	type IdType;
	/// <https://schema.org/BioChemEntity>
	fn find_bio_chem_entity_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBioChemEntityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_bio_chem_entity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BIO_CHEM_ENTITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BIO_CHEM_ENTITY_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBioChemEntityIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_bio_chem_entity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BIO_CHEM_ENTITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BIO_CHEM_ENTITY_IRI_HTTPS,
			})
		}
	}
}
