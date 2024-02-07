/// <https://schema.org/Osteopathic>
pub trait FindOsteopathicIds {
	type IdType;
	/// <https://schema.org/Osteopathic>
	fn find_osteopathic_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOsteopathicIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_osteopathic_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OSTEOPATHIC_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OSTEOPATHIC_IRI_HTTPS,
			})
		}
	}
}
