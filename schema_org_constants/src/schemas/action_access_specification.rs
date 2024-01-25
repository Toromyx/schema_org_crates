/// <https://schema.org/ActionAccessSpecification>
pub const ACTION_ACCESS_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/ActionAccessSpecification";
/// <https://schema.org/ActionAccessSpecification>
pub const ACTION_ACCESS_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/ActionAccessSpecification";
/// <https://schema.org/ActionAccessSpecification>
pub const ACTION_ACCESS_SPECIFICATION_LABEL: &str = "ActionAccessSpecification";
pub struct ActionAccessSpecificationIri;
impl PartialEq<&str> for ActionAccessSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTION_ACCESS_SPECIFICATION_IRI_HTTP
			|| *other == ACTION_ACCESS_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<ActionAccessSpecificationIri> for &str {
	fn eq(&self, other: &ActionAccessSpecificationIri) -> bool {
		*self == ACTION_ACCESS_SPECIFICATION_IRI_HTTP
			|| *self == ACTION_ACCESS_SPECIFICATION_IRI_HTTPS
	}
}
pub struct ActionAccessSpecificationIriOrLabel;
impl PartialEq<&str> for ActionAccessSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionAccessSpecificationIri || *other == ACTION_ACCESS_SPECIFICATION_LABEL
	}
}
impl PartialEq<ActionAccessSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &ActionAccessSpecificationIriOrLabel) -> bool {
		*self == ActionAccessSpecificationIri || *self == ACTION_ACCESS_SPECIFICATION_LABEL
	}
}
