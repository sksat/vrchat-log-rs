use std::ops::Deref;

use crate::log;
use crate::DateTime;

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
    pub nonce: Option<String>,
}

#[derive(Debug)]
pub struct InstanceLog {
    pub instance: Instance,
    pub enter: Option<DateTime>,
    pub join: Option<DateTime>,
    pub join_or_create: Option<DateTime>,
    pub joined: Option<DateTime>,
    pub left: Option<DateTime>,
}

#[derive(Debug)]
pub struct InstanceLogList(Vec<InstanceLog>);

#[derive(Debug)]
pub enum InstanceParseError {
    IdIsNotNumber(String),
    InvalidElement(String),
    UnknownElement(String),
    Invalid(String),
    InvalidReqInvite(InstanceType),
}

/// parse instance string like:
/// wrld_f8ff20cd-5310-4257-ade8-c3fd6ae95436:98257~friends(usr_f8229b4f-794c-4a94-bf5d-d21f3fc0daf5)~nonce(1104791A7210A68C4AE6C869F9B8944FFE00AA9425AA349926F5EDDB93DCB297)
pub fn parse_instance(s: &str) -> Result<Instance, InstanceParseError> {
    let s = s
        .strip_prefix("wrld_")
        .ok_or(InstanceParseError::Invalid(s.to_string()))?;

    // split element
    let mut elem: Vec<&str> = s.split('~').collect();
    elem.retain(|w| !w.is_empty());

    let mut typ = InstanceType::Unknown;
    if elem.len() == 1 {
        // wrld_hogehgoe:id
        typ = InstanceType::Public;
    }
    let mut it = elem.iter();

    // parse world id and instance number
    let world = it
        .next()
        .ok_or(InstanceParseError::Invalid(s.to_string()))?;
    let w: Vec<&str> = world.split(':').collect();
    if w.len() != 2 {
        Err(InstanceParseError::InvalidElement(world.to_string()))?
    }
    let id = w[1].parse().unwrap();

    let world = World {
        id: w[0].to_string(),
        name: "".to_string(),
    };

    let mut owner = None;
    let mut nonce = None;

    for e in it {
        if e.is_empty() {
            continue;
        }

        //println!("{}", e);
        let e: Vec<&str> = e.split('(').collect();

        // no args
        if e.len() == 1 {
            match e[0] {
                "canRequestInvite" => {
                    typ = match typ {
                        InstanceType::Unknown => InstanceType::InvitePlus, // launch from vrchat.com
                        InstanceType::Invite => InstanceType::InvitePlus,  // launch from app
                        _ => Err(InstanceParseError::InvalidReqInvite(typ))?,
                    };
                }
                _ => Err(InstanceParseError::UnknownElement(e[0].to_string()))?,
            };
            continue;
        }

        // has args
        let arg = &e[1];
        let arg = &arg[..arg.len() - 1];
        match e[0] {
            "private" => {
                let o = arg.strip_prefix("usr_").unwrap();
                owner = Some(o.to_string());
                typ = InstanceType::Invite;
            }
            "hidden" => {
                let o = arg.strip_prefix("usr_").unwrap();
                owner = Some(o.to_string());
                typ = InstanceType::FriendPlus;
            }
            "friends" => {
                let o = arg.strip_prefix("usr_").unwrap();
                owner = Some(o.to_string());
                typ = InstanceType::Friends;
            }
            "nonce" => {
                nonce = Some(arg.to_string());
            }
            _ => Err(InstanceParseError::UnknownElement(e[0].to_string()))?,
        }
    }

    Ok(Instance {
        world,
        id,
        owner,
        typ,
        nonce,
    })
}

impl From<&Vec<crate::LogEnum>> for InstanceLogList {
    fn from(from: &Vec<crate::LogEnum>) -> InstanceLogList {
        let mut v = Vec::new();
        let mut log = from.iter();

        let mut ilog: Option<InstanceLog> = None;
        let mut world_name = None;
        let mut enter = None;
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
                enter = Some(l.date);
                continue;
            }
            if msg.starts_with("Joining wrld_") {
                let msg = msg.strip_prefix("Joining ").unwrap();
                let mut instance = parse_instance(msg).unwrap();
                if let Some(ref name) = world_name {
                    instance.world.name = name.to_string();
                }
                ilog = Some(InstanceLog {
                    instance,
                    enter,
                    join: Some(l.date),
                    join_or_create: None,
                    joined: None,
                    left: None,
                });
            }

            if let Some(name) = msg.strip_prefix("Joining or Creating Room: ") {
                if let Some(ref mut ilog) = ilog {
                    let wn = &mut ilog.instance.world.name;
                    if wn.is_empty() {
                        *wn = name.to_string();
                    } else if wn != name {
                        panic!("somthing wrong");
                    }
                    ilog.join_or_create = Some(l.date);
                }
            }

            if msg == "Successfully joined room" {
                if let Some(ref mut ilog) = ilog {
                    ilog.joined = Some(l.date);
                }
            }

            if msg == "Successfully left room" {
                if let Some(mut ilog) = ilog {
                    ilog.left = Some(l.date);
                    v.push(ilog);
                }
                ilog = None;
            }
        }
        // no left
        if let Some(ilog) = ilog {
            if v.is_empty() || v.last().unwrap().instance.id != ilog.instance.id {
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
