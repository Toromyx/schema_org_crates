/// <https://schema.org/skills>
pub const SKILLS_PROPERTY_IRI_HTTP: &str = "http://schema.org/skills";
/// <https://schema.org/skills>
pub const SKILLS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/skills";
/// <https://schema.org/skills>
pub const SKILLS_PROPERTY_LABEL: &str = "skills";
pub struct SkillsPropertyIri;
impl PartialEq<&str> for SkillsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SKILLS_PROPERTY_IRI_HTTP || *other == SKILLS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SkillsPropertyIri> for &str {
	fn eq(&self, other: &SkillsPropertyIri) -> bool {
		*self == SKILLS_PROPERTY_IRI_HTTP || *self == SKILLS_PROPERTY_IRI_HTTPS
	}
}
pub struct SkillsPropertyIriOrLabel;
impl PartialEq<&str> for SkillsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SkillsPropertyIri || *other == SKILLS_PROPERTY_LABEL
	}
}
impl PartialEq<SkillsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SkillsPropertyIriOrLabel) -> bool {
		*self == SkillsPropertyIri || *self == SKILLS_PROPERTY_LABEL
	}
}
