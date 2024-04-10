/// <https://schema.org/ContactPointOption>
pub trait FindContactPointOptionIds {
	type IdType;
	/// <https://schema.org/ContactPointOption>
	fn find_contact_point_option_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindContactPointOptionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_contact_point_option_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CONTACT_POINT_OPTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CONTACT_POINT_OPTION_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindContactPointOptionIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_contact_point_option_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CONTACT_POINT_OPTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CONTACT_POINT_OPTION_IRI_HTTPS,
			})
		}
	}
}
