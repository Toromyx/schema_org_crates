/// <https://schema.org/BarOrPub>
pub trait FindBarOrPubIds {
	type IdType;
	/// <https://schema.org/BarOrPub>
	fn find_bar_or_pub_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBarOrPubIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_bar_or_pub_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BAR_OR_PUB_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BAR_OR_PUB_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBarOrPubIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_bar_or_pub_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BAR_OR_PUB_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BAR_OR_PUB_IRI_HTTPS,
			})
		}
	}
}
