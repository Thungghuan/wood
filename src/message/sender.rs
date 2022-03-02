use serde::{Deserialize, Serialize};

use super::ChatroomType;

pub trait Sender {
    fn chatroom_type(&self) -> ChatroomType;
    fn chatroom_id(&self) -> i32;
    fn chatroom_name(&self) -> String;

    fn sender_id(&self) -> i32;
    fn sender_nickname(&self) -> String;
}

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

impl Sender for FriendSender {
    fn chatroom_type(&self) -> ChatroomType {
        ChatroomType::Friend
    }

    fn chatroom_id(&self) -> i32 {
        self.id
    }

    // chatroom name is only for group message
    fn chatroom_name(&self) -> String {
        "".to_string()
    }

    fn sender_id(&self) -> i32 {
        self.id
    }

    fn sender_nickname(&self) -> String {
        self.nickname.clone()
    }
}

impl Sender for GroupSender {
    fn chatroom_type(&self) -> ChatroomType {
        ChatroomType::Group
    }

    fn chatroom_id(&self) -> i32 {
        self.group.id
    }

    fn chatroom_name(&self) -> String {
        self.group.name.clone()
    }

    fn sender_id(&self) -> i32 {
        self.id
    }

    fn sender_nickname(&self) -> String {
        self.member_name.clone()
    }
}

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
