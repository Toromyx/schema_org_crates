/// <https://schema.org/HyperTocEntry>
pub trait FindHyperTocEntryIds {
	type IdType;
	fn find_hyper_toc_entry_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHyperTocEntryIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_hyper_toc_entry_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HYPER_TOC_ENTRY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HYPER_TOC_ENTRY_IRI_HTTPS,
			})
		}
	}
}
