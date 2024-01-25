/// <https://schema.org/Bone>
pub const BONE_IRI_HTTP: &str = "http://schema.org/Bone";
/// <https://schema.org/Bone>
pub const BONE_IRI_HTTPS: &str = "https://schema.org/Bone";
/// <https://schema.org/Bone>
pub const BONE_LABEL: &str = "Bone";
pub struct BoneIri;
impl PartialEq<&str> for BoneIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BONE_IRI_HTTP || *other == BONE_IRI_HTTPS
	}
}
impl PartialEq<BoneIri> for &str {
	fn eq(&self, other: &BoneIri) -> bool {
		*self == BONE_IRI_HTTP || *self == BONE_IRI_HTTPS
	}
}
pub struct BoneIriOrLabel;
impl PartialEq<&str> for BoneIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BoneIri || *other == BONE_LABEL
	}
}
impl PartialEq<BoneIriOrLabel> for &str {
	fn eq(&self, other: &BoneIriOrLabel) -> bool {
		*self == BoneIri || *self == BONE_LABEL
	}
}
