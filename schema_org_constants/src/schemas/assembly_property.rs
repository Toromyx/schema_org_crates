/// <https://schema.org/assembly>
#[deprecated = "This schema is superseded by <https://schema.org/executableLibraryName>."]
pub const ASSEMBLY_PROPERTY_IRI_HTTP: &str = "http://schema.org/assembly";
/// <https://schema.org/assembly>
#[deprecated = "This schema is superseded by <https://schema.org/executableLibraryName>."]
pub const ASSEMBLY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/assembly";
/// <https://schema.org/assembly>
#[deprecated = "This schema is superseded by <https://schema.org/executableLibraryName>."]
pub const ASSEMBLY_PROPERTY_LABEL: &str = "assembly";
pub struct AssemblyPropertyIri;
impl PartialEq<&str> for AssemblyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSEMBLY_PROPERTY_IRI_HTTP || *other == ASSEMBLY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssemblyPropertyIri> for &str {
	fn eq(&self, other: &AssemblyPropertyIri) -> bool {
		*self == ASSEMBLY_PROPERTY_IRI_HTTP || *self == ASSEMBLY_PROPERTY_IRI_HTTPS
	}
}
pub struct AssemblyPropertyIriOrLabel;
impl PartialEq<&str> for AssemblyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssemblyPropertyIri || *other == ASSEMBLY_PROPERTY_LABEL
	}
}
impl PartialEq<AssemblyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssemblyPropertyIriOrLabel) -> bool {
		*self == AssemblyPropertyIri || *self == ASSEMBLY_PROPERTY_LABEL
	}
}
