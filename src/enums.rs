#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType {
    AddToListenersRequest,
    MemberIn,
    MemberOut,
    RemoveFromListeners,
    ServerOn
}
