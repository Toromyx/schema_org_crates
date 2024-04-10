/// <https://schema.org/VeterinaryCare>
pub trait FindVeterinaryCareIds {
	type IdType;
	/// <https://schema.org/VeterinaryCare>
	fn find_veterinary_care_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVeterinaryCareIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_veterinary_care_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VETERINARY_CARE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VETERINARY_CARE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVeterinaryCareIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_veterinary_care_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VETERINARY_CARE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VETERINARY_CARE_IRI_HTTPS,
			})
		}
	}
}
