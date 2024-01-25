/// <https://schema.org/Joint>
pub const JOINT_IRI_HTTP: &str = "http://schema.org/Joint";
/// <https://schema.org/Joint>
pub const JOINT_IRI_HTTPS: &str = "https://schema.org/Joint";
/// <https://schema.org/Joint>
pub const JOINT_LABEL: &str = "Joint";
pub struct JointIri;
impl PartialEq<&str> for JointIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOINT_IRI_HTTP || *other == JOINT_IRI_HTTPS
	}
}
impl PartialEq<JointIri> for &str {
	fn eq(&self, other: &JointIri) -> bool {
		*self == JOINT_IRI_HTTP || *self == JOINT_IRI_HTTPS
	}
}
pub struct JointIriOrLabel;
impl PartialEq<&str> for JointIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JointIri || *other == JOINT_LABEL
	}
}
impl PartialEq<JointIriOrLabel> for &str {
	fn eq(&self, other: &JointIriOrLabel) -> bool {
		*self == JointIri || *self == JOINT_LABEL
	}
}
