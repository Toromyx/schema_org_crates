/// <https://schema.org/tool>
pub const TOOL_PROPERTY_IRI_HTTP: &str = "http://schema.org/tool";
/// <https://schema.org/tool>
pub const TOOL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tool";
/// <https://schema.org/tool>
pub const TOOL_PROPERTY_LABEL: &str = "tool";
pub struct ToolPropertyIri;
impl PartialEq<&str> for ToolPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOOL_PROPERTY_IRI_HTTP || *other == TOOL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ToolPropertyIri> for &str {
	fn eq(&self, other: &ToolPropertyIri) -> bool {
		*self == TOOL_PROPERTY_IRI_HTTP || *self == TOOL_PROPERTY_IRI_HTTPS
	}
}
pub struct ToolPropertyIriOrLabel;
impl PartialEq<&str> for ToolPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ToolPropertyIri || *other == TOOL_PROPERTY_LABEL
	}
}
impl PartialEq<ToolPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ToolPropertyIriOrLabel) -> bool {
		*self == ToolPropertyIri || *self == TOOL_PROPERTY_LABEL
	}
}
