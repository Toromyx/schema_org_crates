/// <https://schema.org/exampleOfWork>
pub const EXAMPLE_OF_WORK_PROPERTY_IRI_HTTP: &str = "http://schema.org/exampleOfWork";
/// <https://schema.org/exampleOfWork>
pub const EXAMPLE_OF_WORK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/exampleOfWork";
/// <https://schema.org/exampleOfWork>
pub const EXAMPLE_OF_WORK_PROPERTY_LABEL: &str = "exampleOfWork";
pub struct ExampleOfWorkPropertyIri;
impl PartialEq<&str> for ExampleOfWorkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXAMPLE_OF_WORK_PROPERTY_IRI_HTTP || *other == EXAMPLE_OF_WORK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExampleOfWorkPropertyIri> for &str {
	fn eq(&self, other: &ExampleOfWorkPropertyIri) -> bool {
		*self == EXAMPLE_OF_WORK_PROPERTY_IRI_HTTP || *self == EXAMPLE_OF_WORK_PROPERTY_IRI_HTTPS
	}
}
pub struct ExampleOfWorkPropertyIriOrLabel;
impl PartialEq<&str> for ExampleOfWorkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExampleOfWorkPropertyIri || *other == EXAMPLE_OF_WORK_PROPERTY_LABEL
	}
}
impl PartialEq<ExampleOfWorkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExampleOfWorkPropertyIriOrLabel) -> bool {
		*self == ExampleOfWorkPropertyIri || *self == EXAMPLE_OF_WORK_PROPERTY_LABEL
	}
}
