/// <https://schema.org/MusculoskeletalExam>
pub trait FindMusculoskeletalExamIds {
	type IdType;
	/// <https://schema.org/MusculoskeletalExam>
	fn find_musculoskeletal_exam_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMusculoskeletalExamIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_musculoskeletal_exam_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MUSCULOSKELETAL_EXAM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MUSCULOSKELETAL_EXAM_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMusculoskeletalExamIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_musculoskeletal_exam_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MUSCULOSKELETAL_EXAM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MUSCULOSKELETAL_EXAM_IRI_HTTPS,
			})
		}
	}
}
