use std::ops::Deref;

use crate::log;

#[derive(Debug)]
pub struct World {
    pub id: String,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum InstanceType {
    Public,
    FriendPlus,
    Friends,
    InvitePlus,
    Invite,

    Unknown,
}

#[derive(Debug)]
pub struct Instance {
    pub world: World,
    pub id: u32,
    pub owner: Option<String>,
    pub typ: InstanceType,
}

#[derive(Debug)]
pub struct InstanceLog {
    pub instance: Instance,
    pub enter: Option<String>,
    pub join: Option<String>,
    pub join_or_create: Option<String>,
    pub left: Option<String>,
}

#[derive(Debug)]
pub struct InstanceLogList(Vec<InstanceLog>);

/// parse instance string like:
/// wrld_f8ff20cd-5310-4257-ade8-c3fd6ae95436:98257~friends(usr_f8229b4f-794c-4a94-bf5d-d21f3fc0daf5)~nonce(1104791A7210A68C4AE6C869F9B8944FFE00AA9425AA349926F5EDDB93DCB297)
pub fn parse_instance(s: &str) -> Result<Instance, ()> {
    let s = s.strip_prefix("wrld_").unwrap();
    let mut s: Vec<&str> = s.split(&[':', '~', '(', ')'][..]).collect();
    s.retain(|w| !w.is_empty());
    //println!("world: {:?}", s);

    let world = World {
        id: s[0].to_string(),
        name: "".to_string(),
    };
    let id = s[1].parse().unwrap();

    if s.len() == 2 {
        return Ok(Instance {
            world,
            id,
            owner: None,
            typ: InstanceType::Public,
        });
    }

    let owner = s[3].strip_prefix("usr_").map(|u| u.to_string());

    let typ = match s[2] {
        "private" => {
            if s[4] == "canRequestInvite" {
                InstanceType::InvitePlus
            } else {
                InstanceType::Invite
            }
        }
        "friends" => InstanceType::Friends,
        "hidden" => InstanceType::FriendPlus,
        _ => InstanceType::Unknown,
    };

    Ok(Instance {
        world,
        id,
        owner,
        typ,
    })
}

impl From<&Vec<crate::LogEnum>> for InstanceLogList {
    fn from(from: &Vec<crate::LogEnum>) -> InstanceLogList {
        let mut v = Vec::new();
        let mut log = from.iter();

        let mut ilog: Option<InstanceLog> = None;
        let mut world_name = None;
        let mut enter = None;
        let mut left = None;
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
                world_name = Some(name.to_string());
                enter = Some(l.date.clone());
                continue;
            }
            if msg.starts_with("Joining wrld_") {
                let msg = msg.strip_prefix("Joining ").unwrap();
                let instance = parse_instance(msg).unwrap();
                ilog = Some(InstanceLog {
                    instance,
                    enter: None,
                    join: None,
                    join_or_create: None,
                    left: None,
                });
            }

            if msg == "Successfully left room" {
                left = Some(l.date.clone());
                v.push(ilog.unwrap());
                ilog = None;
            }
        }
        // no left
        if let Some(ilog) = ilog {
            if v.last().unwrap().instance.id != ilog.instance.id {
                v.push(ilog);
            }
        }

        v.into()
    }
}

impl Deref for InstanceLogList {
    type Target = Vec<InstanceLog>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Into<InstanceLogList> for Vec<InstanceLog> {
    fn into(self) -> InstanceLogList {
        InstanceLogList(self)
    }
}
