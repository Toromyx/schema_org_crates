/// <https://schema.org/quest>
pub const QUEST_PROPERTY_IRI_HTTP: &str = "http://schema.org/quest";
/// <https://schema.org/quest>
pub const QUEST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/quest";
/// <https://schema.org/quest>
pub const QUEST_PROPERTY_LABEL: &str = "quest";
pub struct QuestPropertyIri;
impl PartialEq<&str> for QuestPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUEST_PROPERTY_IRI_HTTP || *other == QUEST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<QuestPropertyIri> for &str {
	fn eq(&self, other: &QuestPropertyIri) -> bool {
		*self == QUEST_PROPERTY_IRI_HTTP || *self == QUEST_PROPERTY_IRI_HTTPS
	}
}
pub struct QuestPropertyIriOrLabel;
impl PartialEq<&str> for QuestPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuestPropertyIri || *other == QUEST_PROPERTY_LABEL
	}
}
impl PartialEq<QuestPropertyIriOrLabel> for &str {
	fn eq(&self, other: &QuestPropertyIriOrLabel) -> bool {
		*self == QuestPropertyIri || *self == QUEST_PROPERTY_LABEL
	}
}
