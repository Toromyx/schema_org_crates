/// <https://schema.org/imagingTechnique>
pub const IMAGING_TECHNIQUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/imagingTechnique";
/// <https://schema.org/imagingTechnique>
pub const IMAGING_TECHNIQUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/imagingTechnique";
/// <https://schema.org/imagingTechnique>
pub const IMAGING_TECHNIQUE_PROPERTY_LABEL: &str = "imagingTechnique";
pub struct ImagingTechniquePropertyIri;
impl PartialEq<&str> for ImagingTechniquePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IMAGING_TECHNIQUE_PROPERTY_IRI_HTTP
			|| *other == IMAGING_TECHNIQUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ImagingTechniquePropertyIri> for &str {
	fn eq(&self, other: &ImagingTechniquePropertyIri) -> bool {
		*self == IMAGING_TECHNIQUE_PROPERTY_IRI_HTTP
			|| *self == IMAGING_TECHNIQUE_PROPERTY_IRI_HTTPS
	}
}
pub struct ImagingTechniquePropertyIriOrLabel;
impl PartialEq<&str> for ImagingTechniquePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ImagingTechniquePropertyIri || *other == IMAGING_TECHNIQUE_PROPERTY_LABEL
	}
}
impl PartialEq<ImagingTechniquePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ImagingTechniquePropertyIriOrLabel) -> bool {
		*self == ImagingTechniquePropertyIri || *self == IMAGING_TECHNIQUE_PROPERTY_LABEL
	}
}
