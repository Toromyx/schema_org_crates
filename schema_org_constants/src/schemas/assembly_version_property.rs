/// <https://schema.org/assemblyVersion>
pub const ASSEMBLY_VERSION_PROPERTY_IRI_HTTP: &str = "http://schema.org/assemblyVersion";
/// <https://schema.org/assemblyVersion>
pub const ASSEMBLY_VERSION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/assemblyVersion";
/// <https://schema.org/assemblyVersion>
pub const ASSEMBLY_VERSION_PROPERTY_LABEL: &str = "assemblyVersion";
pub struct AssemblyVersionPropertyIri;
impl PartialEq<&str> for AssemblyVersionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSEMBLY_VERSION_PROPERTY_IRI_HTTP
			|| *other == ASSEMBLY_VERSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssemblyVersionPropertyIri> for &str {
	fn eq(&self, other: &AssemblyVersionPropertyIri) -> bool {
		*self == ASSEMBLY_VERSION_PROPERTY_IRI_HTTP || *self == ASSEMBLY_VERSION_PROPERTY_IRI_HTTPS
	}
}
pub struct AssemblyVersionPropertyIriOrLabel;
impl PartialEq<&str> for AssemblyVersionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssemblyVersionPropertyIri || *other == ASSEMBLY_VERSION_PROPERTY_LABEL
	}
}
impl PartialEq<AssemblyVersionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssemblyVersionPropertyIriOrLabel) -> bool {
		*self == AssemblyVersionPropertyIri || *self == ASSEMBLY_VERSION_PROPERTY_LABEL
	}
}
