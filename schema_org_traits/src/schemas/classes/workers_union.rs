/// <https://schema.org/WorkersUnion>
pub trait FindWorkersUnionIds {
	type IdType;
	/// <https://schema.org/WorkersUnion>
	fn find_workers_union_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWorkersUnionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_workers_union_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WORKERS_UNION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WORKERS_UNION_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWorkersUnionIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_workers_union_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WORKERS_UNION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WORKERS_UNION_IRI_HTTPS,
			})
		}
	}
}
