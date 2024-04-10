/// <https://schema.org/StupidType>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub trait FindStupidTypeIds {
	type IdType;
	/// <https://schema.org/StupidType>
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
	fn find_stupid_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindStupidTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_stupid_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::STUPID_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::STUPID_TYPE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindStupidTypeIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_stupid_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::STUPID_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::STUPID_TYPE_IRI_HTTPS,
			})
		}
	}
}
