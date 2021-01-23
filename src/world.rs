use std::ops::Deref;

use crate::log;

#[derive(Debug)]
pub struct World {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub enum InstanceType {
    Public,
    FriendPlus,
    Friends,
    InvitePlus,
    Invite,

    Unknown,
}

#[derive(Debug)]
pub struct InstanceLog(Vec<Instance>);

#[derive(Debug)]
pub struct Instance {
    pub world: World,
    pub typ: InstanceType,

    pub enter: Option<String>,
    pub join: Option<String>,
    pub join_or_create: Option<String>,
    pub left: Option<String>,
}

impl From<&Vec<crate::LogEnum>> for InstanceLog {
    fn from(from: &Vec<crate::LogEnum>) -> InstanceLog {
        let mut v = Vec::new();
        let mut log = from.iter();
        let mut inst = Instance::default();
        loop {
            let l = log.next();
            if l.is_none() {
                break;
            }
            let l = l.unwrap().as_log();
            if l.is_none() {
                continue;
            }
            let l = l.unwrap();
            if l.typ != log::Type::RoomManager {
                continue;
            }

            let msg = &l.msg[0];
            if msg == "Clearing Room Metadata" || msg.starts_with("Room metadata") {
                continue;
            }

            if let Some(name) = msg.strip_prefix("Entering Room: ") {
                inst.world.name = name.to_string();
                inst.enter = Some(l.date.clone());
            }
            if msg == "Successfully left room" {
                inst.left = Some(l.date.clone());
                v.push(inst);
                inst = Instance::default();
            }
        }

        v.into()
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            world: World {
                name: "".to_string(),
                id: "".to_string(),
            },
            typ: InstanceType::Unknown,
            enter: None,
            join: None,
            join_or_create: None,
            left: None,
        }
    }
}

impl Deref for InstanceLog {
    type Target = Vec<Instance>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Into<InstanceLog> for Vec<Instance> {
    fn into(self) -> InstanceLog {
        InstanceLog(self)
    }
}
