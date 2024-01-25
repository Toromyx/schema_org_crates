/// <https://schema.org/CDCPMDRecord>
pub const CDCPMD_RECORD_IRI_HTTP: &str = "http://schema.org/CDCPMDRecord";
/// <https://schema.org/CDCPMDRecord>
pub const CDCPMD_RECORD_IRI_HTTPS: &str = "https://schema.org/CDCPMDRecord";
/// <https://schema.org/CDCPMDRecord>
pub const CDCPMD_RECORD_LABEL: &str = "CDCPMDRecord";
pub struct CdcpmdRecordIri;
impl PartialEq<&str> for CdcpmdRecordIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CDCPMD_RECORD_IRI_HTTP || *other == CDCPMD_RECORD_IRI_HTTPS
	}
}
impl PartialEq<CdcpmdRecordIri> for &str {
	fn eq(&self, other: &CdcpmdRecordIri) -> bool {
		*self == CDCPMD_RECORD_IRI_HTTP || *self == CDCPMD_RECORD_IRI_HTTPS
	}
}
pub struct CdcpmdRecordIriOrLabel;
impl PartialEq<&str> for CdcpmdRecordIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CdcpmdRecordIri || *other == CDCPMD_RECORD_LABEL
	}
}
impl PartialEq<CdcpmdRecordIriOrLabel> for &str {
	fn eq(&self, other: &CdcpmdRecordIriOrLabel) -> bool {
		*self == CdcpmdRecordIri || *self == CDCPMD_RECORD_LABEL
	}
}
