/// <https://schema.org/monoisotopicMolecularWeight>
pub const MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/monoisotopicMolecularWeight";
/// <https://schema.org/monoisotopicMolecularWeight>
pub const MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/monoisotopicMolecularWeight";
/// <https://schema.org/monoisotopicMolecularWeight>
pub const MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_LABEL: &str = "monoisotopicMolecularWeight";
pub struct MonoisotopicMolecularWeightPropertyIri;
impl PartialEq<&str> for MonoisotopicMolecularWeightPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_IRI_HTTP
			|| *other == MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MonoisotopicMolecularWeightPropertyIri> for &str {
	fn eq(&self, other: &MonoisotopicMolecularWeightPropertyIri) -> bool {
		*self == MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_IRI_HTTP
			|| *self == MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_IRI_HTTPS
	}
}
pub struct MonoisotopicMolecularWeightPropertyIriOrLabel;
impl PartialEq<&str> for MonoisotopicMolecularWeightPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MonoisotopicMolecularWeightPropertyIri
			|| *other == MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_LABEL
	}
}
impl PartialEq<MonoisotopicMolecularWeightPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MonoisotopicMolecularWeightPropertyIriOrLabel) -> bool {
		*self == MonoisotopicMolecularWeightPropertyIri
			|| *self == MONOISOTOPIC_MOLECULAR_WEIGHT_PROPERTY_LABEL
	}
}
