use vrchat_log::world::*;

#[test]
fn parse_public() {
    let s = "wrld_f8ff20cd-5310-4257-ade8-c3fd6ae95436:25280";
    let i = parse_instance(s).unwrap();
    assert_eq!(i.world.id, "f8ff20cd-5310-4257-ade8-c3fd6ae95436");
    assert_eq!(i.typ, InstanceType::Public);
}

#[test]
fn parse_friendplus() {
    let s = "wrld_f8ff20cd-5310-4257-ade8-c3fd6ae95436:96401~hidden(usr_f8229b4f-794c-4a94-bf5d-d21f3fc0daf5)~nonce(0D3A070422B3CD8ADEA8692F2F3C7847800707CE90F38E27E503EF19485A7CB1)";
    let i = parse_instance(s).unwrap();
    assert_eq!(i.world.id, "f8ff20cd-5310-4257-ade8-c3fd6ae95436");
    assert_eq!(i.typ, InstanceType::FriendPlus);
    assert_eq!(
        i.owner,
        Some("f8229b4f-794c-4a94-bf5d-d21f3fc0daf5".to_string())
    )
}

#[test]
fn parse_friends() {
    let s = "wrld_f8ff20cd-5310-4257-ade8-c3fd6ae95436:88109~friends(usr_f8229b4f-794c-4a94-bf5d-d21f3fc0daf5)~nonce(7E2FC1233407594AA0FFD0CF53F93FF4EF8FE7501A671E0B1D55475CC0F93166)";
    let i = parse_instance(s).unwrap();
    assert_eq!(i.world.id, "f8ff20cd-5310-4257-ade8-c3fd6ae95436");
    assert_eq!(i.typ, InstanceType::Friends);
    assert_eq!(
        i.owner,
        Some("f8229b4f-794c-4a94-bf5d-d21f3fc0daf5".to_string())
    )
}

#[test]
fn parse_inviteplus() {
    let s = "wrld_f8ff20cd-5310-4257-ade8-c3fd6ae95436:47859~private(usr_f8229b4f-794c-4a94-bf5d-d21f3fc0daf5)~canRequestInvite~nonce(0D35286CB581DB03D6812E3E92DE13C3BADAF1355FC1D030F9B3E2F4C985D517)";
    let i = parse_instance(s).unwrap();
    assert_eq!(i.world.id, "f8ff20cd-5310-4257-ade8-c3fd6ae95436");
    assert_eq!(i.typ, InstanceType::InvitePlus);
    assert_eq!(
        i.owner,
        Some("f8229b4f-794c-4a94-bf5d-d21f3fc0daf5".to_string())
    )
}

#[test]
fn parse_inviteplus_web() {
    let s = "wrld_14a3b8fa-f706-466f-ab83-bb48610d6904:84198~private(usr_f8229b4f-794c-4a94-bf5d-d21f3fc0daf5)~nonce(221a42fa-fc06-427c-9a5c-09313b8f00fa)~canRequestInvite";
    let i = parse_instance(s).unwrap();
    assert_eq!(i.world.id, "14a3b8fa-f706-466f-ab83-bb48610d6904");
    assert_eq!(i.typ, InstanceType::InvitePlus);
    assert_eq!(
        i.owner,
        Some("f8229b4f-794c-4a94-bf5d-d21f3fc0daf5".to_string())
    )
}

#[test]
fn parse_inviteonly() {
    let s = "wrld_f8ff20cd-5310-4257-ade8-c3fd6ae95436:43386~private(usr_f8229b4f-794c-4a94-bf5d-d21f3fc0daf5)~nonce(0AA407E5E63DCAD3FDF660148A8E261A1AE36E7066C91EC1CB9A25E858E745A8)";
    let i = parse_instance(s).unwrap();
    assert_eq!(i.world.id, "f8ff20cd-5310-4257-ade8-c3fd6ae95436");
    assert_eq!(i.typ, InstanceType::Invite);
    assert_eq!(
        i.owner,
        Some("f8229b4f-794c-4a94-bf5d-d21f3fc0daf5".to_string())
    )
}
