use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MainCityObjectTemplate {
    #[serde(rename = "TagID")]
    pub tag_id: i32,
    #[serde(rename = "NPCID")]
    pub npc_id: i32,
    #[serde(rename = "CreatePosition")]
    pub create_position: String,
    #[serde(rename = "CreateType")]
    pub create_type: i32,
    #[serde(rename = "DefaultInteractIDs")]
    pub default_interact_ids: Vec<i32>,
    #[serde(rename = "InteractName")]
    pub interact_name: Option<String>,
    #[serde(rename = "InteractShape")]
    pub interact_shape: i32,
    #[serde(rename = "InteractScale")]
    pub interact_scale: String,
    #[serde(rename = "FanInteractParam")]
    pub fan_interact_param: Option<String>,
    #[serde(rename = "FocusInteractScale")]
    pub focus_interact_scale: f64,
    #[serde(rename = "IgnoreCollider")]
    pub ignore_collider: bool,
    #[serde(rename = "LookIK")]
    pub look_ik: bool,
    #[serde(rename = "NPCLookIK")]
    pub npc_look_ik: bool,
    #[serde(rename = "SceneSoundID")]
    pub scene_sound_id: i32,
    #[serde(rename = "PlayerRotate")]
    pub player_rotate: bool,
    #[serde(rename = "NPCRotate")]
    pub npc_rotate: bool,
    #[serde(rename = "SceneObjectName")]
    pub scene_object_name: Option<String>,
    #[serde(rename = "CameraStoryKey")]
    pub camera_story_key: Option<String>,
    #[serde(rename = "ActionState")]
    pub action_state: i32,
    #[serde(rename = "ColliderState")]
    pub collider_state: Option<String>,
    #[serde(rename = "ItemState")]
    pub item_state: Option<String>,
    #[serde(rename = "ObjectIDs")]
    pub object_ids: Vec<i32>,
    #[serde(rename = "CreateInterval")]
    pub create_interval: i32,
    #[serde(rename = "CreateDelay")]
    pub create_delay: i32,
    #[serde(rename = "NPCIcon")]
    pub npc_icon: Option<String>,
    #[serde(rename = "ActionSwitch")]
    pub action_switch: Option<String>,
}