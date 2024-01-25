/// <https://schema.org/maintainer>
pub const MAINTAINER_PROPERTY_IRI_HTTP: &str = "http://schema.org/maintainer";
/// <https://schema.org/maintainer>
pub const MAINTAINER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/maintainer";
/// <https://schema.org/maintainer>
pub const MAINTAINER_PROPERTY_LABEL: &str = "maintainer";
pub struct MaintainerPropertyIri;
impl PartialEq<&str> for MaintainerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAINTAINER_PROPERTY_IRI_HTTP || *other == MAINTAINER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaintainerPropertyIri> for &str {
	fn eq(&self, other: &MaintainerPropertyIri) -> bool {
		*self == MAINTAINER_PROPERTY_IRI_HTTP || *self == MAINTAINER_PROPERTY_IRI_HTTPS
	}
}
pub struct MaintainerPropertyIriOrLabel;
impl PartialEq<&str> for MaintainerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaintainerPropertyIri || *other == MAINTAINER_PROPERTY_LABEL
	}
}
impl PartialEq<MaintainerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaintainerPropertyIriOrLabel) -> bool {
		*self == MaintainerPropertyIri || *self == MAINTAINER_PROPERTY_LABEL
	}
}
