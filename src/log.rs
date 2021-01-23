use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Type {
    #[strum(disabled)]
    Message, // not []

    #[strum(default)]
    Unimplemented(String),

    API,
    Always,
    AssetBundleDownloadManager,
    AvatarPlayableController,
    GC,
    IkController,
    NetworkManager,
    ObjectInstantiator,
    Player,
    PlayerManager,
    RoomManager,
    SpawnManager,
    USpeaker,
    VRCApplicationSetup,
    VRCAvatarManager,
    VRCFlowManagerVRC,
    VRCFlowNetworkManager,
    VRCHandGrasper,
    VRCVrIkController,
    VRC_AnimationController,
}
