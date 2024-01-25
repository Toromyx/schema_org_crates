/// <https://schema.org/Monday>
pub const MONDAY_IRI_HTTP: &str = "http://schema.org/Monday";
/// <https://schema.org/Monday>
pub const MONDAY_IRI_HTTPS: &str = "https://schema.org/Monday";
/// <https://schema.org/Monday>
pub const MONDAY_LABEL: &str = "Monday";
pub struct MondayIri;
impl PartialEq<&str> for MondayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MONDAY_IRI_HTTP || *other == MONDAY_IRI_HTTPS
	}
}
impl PartialEq<MondayIri> for &str {
	fn eq(&self, other: &MondayIri) -> bool {
		*self == MONDAY_IRI_HTTP || *self == MONDAY_IRI_HTTPS
	}
}
pub struct MondayIriOrLabel;
impl PartialEq<&str> for MondayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MondayIri || *other == MONDAY_LABEL
	}
}
impl PartialEq<MondayIriOrLabel> for &str {
	fn eq(&self, other: &MondayIriOrLabel) -> bool {
		*self == MondayIri || *self == MONDAY_LABEL
	}
}
