/// <https://schema.org/EntryPoint>
pub const ENTRY_POINT_IRI_HTTP: &str = "http://schema.org/EntryPoint";
/// <https://schema.org/EntryPoint>
pub const ENTRY_POINT_IRI_HTTPS: &str = "https://schema.org/EntryPoint";
/// <https://schema.org/EntryPoint>
pub const ENTRY_POINT_LABEL: &str = "EntryPoint";
pub struct EntryPointIri;
impl PartialEq<&str> for EntryPointIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENTRY_POINT_IRI_HTTP || *other == ENTRY_POINT_IRI_HTTPS
	}
}
impl PartialEq<EntryPointIri> for &str {
	fn eq(&self, other: &EntryPointIri) -> bool {
		*self == ENTRY_POINT_IRI_HTTP || *self == ENTRY_POINT_IRI_HTTPS
	}
}
pub struct EntryPointIriOrLabel;
impl PartialEq<&str> for EntryPointIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EntryPointIri || *other == ENTRY_POINT_LABEL
	}
}
impl PartialEq<EntryPointIriOrLabel> for &str {
	fn eq(&self, other: &EntryPointIriOrLabel) -> bool {
		*self == EntryPointIri || *self == ENTRY_POINT_LABEL
	}
}
