/// <https://schema.org/AskAction>
pub const ASK_ACTION_IRI_HTTP: &str = "http://schema.org/AskAction";
/// <https://schema.org/AskAction>
pub const ASK_ACTION_IRI_HTTPS: &str = "https://schema.org/AskAction";
/// <https://schema.org/AskAction>
pub const ASK_ACTION_LABEL: &str = "AskAction";
pub struct AskActionIri;
impl PartialEq<&str> for AskActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASK_ACTION_IRI_HTTP || *other == ASK_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AskActionIri> for &str {
	fn eq(&self, other: &AskActionIri) -> bool {
		*self == ASK_ACTION_IRI_HTTP || *self == ASK_ACTION_IRI_HTTPS
	}
}
pub struct AskActionIriOrLabel;
impl PartialEq<&str> for AskActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AskActionIri || *other == ASK_ACTION_LABEL
	}
}
impl PartialEq<AskActionIriOrLabel> for &str {
	fn eq(&self, other: &AskActionIriOrLabel) -> bool {
		*self == AskActionIri || *self == ASK_ACTION_LABEL
	}
}
