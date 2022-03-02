use serde::{Deserialize, Serialize};

pub trait Sender {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Permission {
    OWNER,
    ADMINISTRATOR,
    MEMBER,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub permission: Permission,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FriendSender {
    pub id: i32,
    pub nickname: String,
    pub remark: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GroupSender {
    pub id: i32,
    pub member_name: String,
    pub permission: Permission,
    pub group: Group,
}

// TODO: implement the `Sender` trait.
impl Sender for FriendSender {}
impl Sender for GroupSender {}

#[test]
fn check_group_sender_deserialize_result() {
    let resp = r#"
    {
        "sender": {
            "id":20211113,
            "memberName":"Thungghuan",
            "specialTitle":"",
            "permission":"OWNER",
            "joinTimestamp":20211113,
            "lastSpeakTimestamp":20211113,
            "muteTimeRemaining":0,
            "group": {
                "id":20211113,
                "name":"木木",
                "permission":"ADMINISTRATOR"
            }
        }
    }"#;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct GroupSenderStruct {
        sender: GroupSender,
    }

    let group = Group {
        id: 20211113,
        name: "木木".to_string(),
        permission: Permission::ADMINISTRATOR,
    };

    let group_sender = GroupSender {
        id: 20211113,
        member_name: "Thungghuan".to_string(),
        permission: Permission::OWNER,
        group,
    };

    let group_sender_struct = GroupSenderStruct {
        sender: group_sender,
    };

    assert_eq!(
        serde_json::from_str::<GroupSenderStruct>(&resp).unwrap(),
        group_sender_struct
    );
}
