/// <https://schema.org/BenefitsHealthAspect>
pub trait FindBenefitsHealthAspectIds {
	type IdType;
	/// <https://schema.org/BenefitsHealthAspect>
	fn find_benefits_health_aspect_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBenefitsHealthAspectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_benefits_health_aspect_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BENEFITS_HEALTH_ASPECT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BENEFITS_HEALTH_ASPECT_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBenefitsHealthAspectIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_benefits_health_aspect_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BENEFITS_HEALTH_ASPECT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BENEFITS_HEALTH_ASPECT_IRI_HTTPS,
			})
		}
	}
}
