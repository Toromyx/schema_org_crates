/// <https://schema.org/ContactPage>
pub trait FindContactPageIds {
	type IdType;
	/// <https://schema.org/ContactPage>
	fn find_contact_page_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindContactPageIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_contact_page_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CONTACT_PAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CONTACT_PAGE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindContactPageIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_contact_page_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CONTACT_PAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CONTACT_PAGE_IRI_HTTPS,
			})
		}
	}
}
