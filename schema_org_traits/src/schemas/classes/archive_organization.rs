/// <https://schema.org/ArchiveOrganization>
pub trait FindArchiveOrganizationIds {
	type IdType;
	/// <https://schema.org/ArchiveOrganization>
	fn find_archive_organization_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindArchiveOrganizationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_archive_organization_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ARCHIVE_ORGANIZATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ARCHIVE_ORGANIZATION_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindArchiveOrganizationIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_archive_organization_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ARCHIVE_ORGANIZATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ARCHIVE_ORGANIZATION_IRI_HTTPS,
			})
		}
	}
}
