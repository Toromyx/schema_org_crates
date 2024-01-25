/// <https://schema.org/Conversation>
pub const CONVERSATION_IRI_HTTP: &str = "http://schema.org/Conversation";
/// <https://schema.org/Conversation>
pub const CONVERSATION_IRI_HTTPS: &str = "https://schema.org/Conversation";
/// <https://schema.org/Conversation>
pub const CONVERSATION_LABEL: &str = "Conversation";
pub struct ConversationIri;
impl PartialEq<&str> for ConversationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONVERSATION_IRI_HTTP || *other == CONVERSATION_IRI_HTTPS
	}
}
impl PartialEq<ConversationIri> for &str {
	fn eq(&self, other: &ConversationIri) -> bool {
		*self == CONVERSATION_IRI_HTTP || *self == CONVERSATION_IRI_HTTPS
	}
}
pub struct ConversationIriOrLabel;
impl PartialEq<&str> for ConversationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConversationIri || *other == CONVERSATION_LABEL
	}
}
impl PartialEq<ConversationIriOrLabel> for &str {
	fn eq(&self, other: &ConversationIriOrLabel) -> bool {
		*self == ConversationIri || *self == CONVERSATION_LABEL
	}
}
