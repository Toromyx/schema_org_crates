/// <https://schema.org/conditionsOfAccess>
pub const CONDITIONS_OF_ACCESS_PROPERTY_IRI_HTTP: &str = "http://schema.org/conditionsOfAccess";
/// <https://schema.org/conditionsOfAccess>
pub const CONDITIONS_OF_ACCESS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/conditionsOfAccess";
/// <https://schema.org/conditionsOfAccess>
pub const CONDITIONS_OF_ACCESS_PROPERTY_LABEL: &str = "conditionsOfAccess";
pub struct ConditionsOfAccessPropertyIri;
impl PartialEq<&str> for ConditionsOfAccessPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONDITIONS_OF_ACCESS_PROPERTY_IRI_HTTP
			|| *other == CONDITIONS_OF_ACCESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ConditionsOfAccessPropertyIri> for &str {
	fn eq(&self, other: &ConditionsOfAccessPropertyIri) -> bool {
		*self == CONDITIONS_OF_ACCESS_PROPERTY_IRI_HTTP
			|| *self == CONDITIONS_OF_ACCESS_PROPERTY_IRI_HTTPS
	}
}
pub struct ConditionsOfAccessPropertyIriOrLabel;
impl PartialEq<&str> for ConditionsOfAccessPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConditionsOfAccessPropertyIri || *other == CONDITIONS_OF_ACCESS_PROPERTY_LABEL
	}
}
impl PartialEq<ConditionsOfAccessPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ConditionsOfAccessPropertyIriOrLabel) -> bool {
		*self == ConditionsOfAccessPropertyIri || *self == CONDITIONS_OF_ACCESS_PROPERTY_LABEL
	}
}
