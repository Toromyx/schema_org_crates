/// <https://schema.org/executableLibraryName>
pub const EXECUTABLE_LIBRARY_NAME_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/executableLibraryName";
/// <https://schema.org/executableLibraryName>
pub const EXECUTABLE_LIBRARY_NAME_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/executableLibraryName";
/// <https://schema.org/executableLibraryName>
pub const EXECUTABLE_LIBRARY_NAME_PROPERTY_LABEL: &str = "executableLibraryName";
pub struct ExecutableLibraryNamePropertyIri;
impl PartialEq<&str> for ExecutableLibraryNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXECUTABLE_LIBRARY_NAME_PROPERTY_IRI_HTTP
			|| *other == EXECUTABLE_LIBRARY_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExecutableLibraryNamePropertyIri> for &str {
	fn eq(&self, other: &ExecutableLibraryNamePropertyIri) -> bool {
		*self == EXECUTABLE_LIBRARY_NAME_PROPERTY_IRI_HTTP
			|| *self == EXECUTABLE_LIBRARY_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct ExecutableLibraryNamePropertyIriOrLabel;
impl PartialEq<&str> for ExecutableLibraryNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExecutableLibraryNamePropertyIri
			|| *other == EXECUTABLE_LIBRARY_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<ExecutableLibraryNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExecutableLibraryNamePropertyIriOrLabel) -> bool {
		*self == ExecutableLibraryNamePropertyIri || *self == EXECUTABLE_LIBRARY_NAME_PROPERTY_LABEL
	}
}
