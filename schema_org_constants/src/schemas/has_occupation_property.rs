/// <https://schema.org/hasOccupation>
pub const HAS_OCCUPATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasOccupation";
/// <https://schema.org/hasOccupation>
pub const HAS_OCCUPATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasOccupation";
/// <https://schema.org/hasOccupation>
pub const HAS_OCCUPATION_PROPERTY_LABEL: &str = "hasOccupation";
pub struct HasOccupationPropertyIri;
impl PartialEq<&str> for HasOccupationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_OCCUPATION_PROPERTY_IRI_HTTP || *other == HAS_OCCUPATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasOccupationPropertyIri> for &str {
	fn eq(&self, other: &HasOccupationPropertyIri) -> bool {
		*self == HAS_OCCUPATION_PROPERTY_IRI_HTTP || *self == HAS_OCCUPATION_PROPERTY_IRI_HTTPS
	}
}
pub struct HasOccupationPropertyIriOrLabel;
impl PartialEq<&str> for HasOccupationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasOccupationPropertyIri || *other == HAS_OCCUPATION_PROPERTY_LABEL
	}
}
impl PartialEq<HasOccupationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasOccupationPropertyIriOrLabel) -> bool {
		*self == HasOccupationPropertyIri || *self == HAS_OCCUPATION_PROPERTY_LABEL
	}
}
