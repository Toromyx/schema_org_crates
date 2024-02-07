/// <https://schema.org/SatireOrParodyContent>
pub trait FindSatireOrParodyContentIds {
	type IdType;
	/// <https://schema.org/SatireOrParodyContent>
	fn find_satire_or_parody_content_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSatireOrParodyContentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_satire_or_parody_content_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SATIRE_OR_PARODY_CONTENT_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::SATIRE_OR_PARODY_CONTENT_IRI_HTTPS
				}
			})
		}
	}
}
