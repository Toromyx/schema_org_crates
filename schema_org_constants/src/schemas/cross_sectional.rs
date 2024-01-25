/// <https://schema.org/CrossSectional>
pub const CROSS_SECTIONAL_IRI_HTTP: &str = "http://schema.org/CrossSectional";
/// <https://schema.org/CrossSectional>
pub const CROSS_SECTIONAL_IRI_HTTPS: &str = "https://schema.org/CrossSectional";
/// <https://schema.org/CrossSectional>
pub const CROSS_SECTIONAL_LABEL: &str = "CrossSectional";
pub struct CrossSectionalIri;
impl PartialEq<&str> for CrossSectionalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CROSS_SECTIONAL_IRI_HTTP || *other == CROSS_SECTIONAL_IRI_HTTPS
	}
}
impl PartialEq<CrossSectionalIri> for &str {
	fn eq(&self, other: &CrossSectionalIri) -> bool {
		*self == CROSS_SECTIONAL_IRI_HTTP || *self == CROSS_SECTIONAL_IRI_HTTPS
	}
}
pub struct CrossSectionalIriOrLabel;
impl PartialEq<&str> for CrossSectionalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CrossSectionalIri || *other == CROSS_SECTIONAL_LABEL
	}
}
impl PartialEq<CrossSectionalIriOrLabel> for &str {
	fn eq(&self, other: &CrossSectionalIriOrLabel) -> bool {
		*self == CrossSectionalIri || *self == CROSS_SECTIONAL_LABEL
	}
}
