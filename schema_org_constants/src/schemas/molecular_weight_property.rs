/// <https://schema.org/molecularWeight>
pub const MOLECULAR_WEIGHT_PROPERTY_IRI_HTTP: &str = "http://schema.org/molecularWeight";
/// <https://schema.org/molecularWeight>
pub const MOLECULAR_WEIGHT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/molecularWeight";
/// <https://schema.org/molecularWeight>
pub const MOLECULAR_WEIGHT_PROPERTY_LABEL: &str = "molecularWeight";
pub struct MolecularWeightPropertyIri;
impl PartialEq<&str> for MolecularWeightPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOLECULAR_WEIGHT_PROPERTY_IRI_HTTP
			|| *other == MOLECULAR_WEIGHT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MolecularWeightPropertyIri> for &str {
	fn eq(&self, other: &MolecularWeightPropertyIri) -> bool {
		*self == MOLECULAR_WEIGHT_PROPERTY_IRI_HTTP || *self == MOLECULAR_WEIGHT_PROPERTY_IRI_HTTPS
	}
}
pub struct MolecularWeightPropertyIriOrLabel;
impl PartialEq<&str> for MolecularWeightPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MolecularWeightPropertyIri || *other == MOLECULAR_WEIGHT_PROPERTY_LABEL
	}
}
impl PartialEq<MolecularWeightPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MolecularWeightPropertyIriOrLabel) -> bool {
		*self == MolecularWeightPropertyIri || *self == MOLECULAR_WEIGHT_PROPERTY_LABEL
	}
}
