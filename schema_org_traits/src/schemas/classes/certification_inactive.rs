/// <https://schema.org/CertificationInactive>
pub trait FindCertificationInactiveIds {
	type IdType;
	/// <https://schema.org/CertificationInactive>
	fn find_certification_inactive_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCertificationInactiveIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_certification_inactive_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CERTIFICATION_INACTIVE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CERTIFICATION_INACTIVE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCertificationInactiveIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_certification_inactive_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CERTIFICATION_INACTIVE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CERTIFICATION_INACTIVE_IRI_HTTPS,
			})
		}
	}
}
