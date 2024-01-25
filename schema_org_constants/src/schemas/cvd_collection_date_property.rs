/// <https://schema.org/cvdCollectionDate>
pub const CVD_COLLECTION_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdCollectionDate";
/// <https://schema.org/cvdCollectionDate>
pub const CVD_COLLECTION_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdCollectionDate";
/// <https://schema.org/cvdCollectionDate>
pub const CVD_COLLECTION_DATE_PROPERTY_LABEL: &str = "cvdCollectionDate";
pub struct CvdCollectionDatePropertyIri;
impl PartialEq<&str> for CvdCollectionDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_COLLECTION_DATE_PROPERTY_IRI_HTTP
			|| *other == CVD_COLLECTION_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdCollectionDatePropertyIri> for &str {
	fn eq(&self, other: &CvdCollectionDatePropertyIri) -> bool {
		*self == CVD_COLLECTION_DATE_PROPERTY_IRI_HTTP
			|| *self == CVD_COLLECTION_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdCollectionDatePropertyIriOrLabel;
impl PartialEq<&str> for CvdCollectionDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdCollectionDatePropertyIri || *other == CVD_COLLECTION_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<CvdCollectionDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdCollectionDatePropertyIriOrLabel) -> bool {
		*self == CvdCollectionDatePropertyIri || *self == CVD_COLLECTION_DATE_PROPERTY_LABEL
	}
}
