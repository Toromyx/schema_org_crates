/// <https://schema.org/isAvailableGenerically>
pub const IS_AVAILABLE_GENERICALLY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/isAvailableGenerically";
/// <https://schema.org/isAvailableGenerically>
pub const IS_AVAILABLE_GENERICALLY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isAvailableGenerically";
/// <https://schema.org/isAvailableGenerically>
pub const IS_AVAILABLE_GENERICALLY_PROPERTY_LABEL: &str = "isAvailableGenerically";
pub struct IsAvailableGenericallyPropertyIri;
impl PartialEq<&str> for IsAvailableGenericallyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_AVAILABLE_GENERICALLY_PROPERTY_IRI_HTTP
			|| *other == IS_AVAILABLE_GENERICALLY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsAvailableGenericallyPropertyIri> for &str {
	fn eq(&self, other: &IsAvailableGenericallyPropertyIri) -> bool {
		*self == IS_AVAILABLE_GENERICALLY_PROPERTY_IRI_HTTP
			|| *self == IS_AVAILABLE_GENERICALLY_PROPERTY_IRI_HTTPS
	}
}
pub struct IsAvailableGenericallyPropertyIriOrLabel;
impl PartialEq<&str> for IsAvailableGenericallyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsAvailableGenericallyPropertyIri
			|| *other == IS_AVAILABLE_GENERICALLY_PROPERTY_LABEL
	}
}
impl PartialEq<IsAvailableGenericallyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsAvailableGenericallyPropertyIriOrLabel) -> bool {
		*self == IsAvailableGenericallyPropertyIri
			|| *self == IS_AVAILABLE_GENERICALLY_PROPERTY_LABEL
	}
}
