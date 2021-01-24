use std::str::FromStr;
use strum_macros::EnumString;

use crate::DateTime;

#[derive(Debug)]
pub struct Log {
    pub date: DateTime,
    pub typ: Type,
    pub msg: Vec<String>,
}

impl Log {
    pub fn new(date: DateTime, typ: Option<&str>, msg: Vec<String>) -> Self {
        let typ = if let Some(typ) = typ {
            Type::from_str(typ).unwrap()
        } else {
            Type::Message
        };
        Self { date, typ, msg }
    }
}

#[derive(PartialEq, Debug, EnumString)]
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
