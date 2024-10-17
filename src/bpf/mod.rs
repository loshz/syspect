use prometheus_client::encoding::EncodeLabelSet;

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct DefaultLabels {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub pname: String,
}
