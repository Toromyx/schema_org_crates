/// <https://schema.org/hasMolecularFunction>
pub const HAS_MOLECULAR_FUNCTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasMolecularFunction";
/// <https://schema.org/hasMolecularFunction>
pub const HAS_MOLECULAR_FUNCTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasMolecularFunction";
/// <https://schema.org/hasMolecularFunction>
pub const HAS_MOLECULAR_FUNCTION_PROPERTY_LABEL: &str = "hasMolecularFunction";
pub struct HasMolecularFunctionPropertyIri;
impl PartialEq<&str> for HasMolecularFunctionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_MOLECULAR_FUNCTION_PROPERTY_IRI_HTTP
			|| *other == HAS_MOLECULAR_FUNCTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasMolecularFunctionPropertyIri> for &str {
	fn eq(&self, other: &HasMolecularFunctionPropertyIri) -> bool {
		*self == HAS_MOLECULAR_FUNCTION_PROPERTY_IRI_HTTP
			|| *self == HAS_MOLECULAR_FUNCTION_PROPERTY_IRI_HTTPS
	}
}
pub struct HasMolecularFunctionPropertyIriOrLabel;
impl PartialEq<&str> for HasMolecularFunctionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasMolecularFunctionPropertyIri || *other == HAS_MOLECULAR_FUNCTION_PROPERTY_LABEL
	}
}
impl PartialEq<HasMolecularFunctionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasMolecularFunctionPropertyIriOrLabel) -> bool {
		*self == HasMolecularFunctionPropertyIri || *self == HAS_MOLECULAR_FUNCTION_PROPERTY_LABEL
	}
}
