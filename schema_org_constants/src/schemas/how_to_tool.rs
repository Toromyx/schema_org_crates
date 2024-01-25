/// <https://schema.org/HowToTool>
pub const HOW_TO_TOOL_IRI_HTTP: &str = "http://schema.org/HowToTool";
/// <https://schema.org/HowToTool>
pub const HOW_TO_TOOL_IRI_HTTPS: &str = "https://schema.org/HowToTool";
/// <https://schema.org/HowToTool>
pub const HOW_TO_TOOL_LABEL: &str = "HowToTool";
pub struct HowToToolIri;
impl PartialEq<&str> for HowToToolIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_TO_TOOL_IRI_HTTP || *other == HOW_TO_TOOL_IRI_HTTPS
	}
}
impl PartialEq<HowToToolIri> for &str {
	fn eq(&self, other: &HowToToolIri) -> bool {
		*self == HOW_TO_TOOL_IRI_HTTP || *self == HOW_TO_TOOL_IRI_HTTPS
	}
}
pub struct HowToToolIriOrLabel;
impl PartialEq<&str> for HowToToolIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowToToolIri || *other == HOW_TO_TOOL_LABEL
	}
}
impl PartialEq<HowToToolIriOrLabel> for &str {
	fn eq(&self, other: &HowToToolIriOrLabel) -> bool {
		*self == HowToToolIri || *self == HOW_TO_TOOL_LABEL
	}
}
