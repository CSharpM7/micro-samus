use crate::imports::imports_acmd::*;

#[acmd_script( agent = "samus", script = "effect_ice_lance", category = ACMD_EFFECT)]
unsafe fn effect_ice_lance(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("handr"), 5.0, 0.0, 0.0, 0, 0, -90, 0.2, true);
        LAST_EFFECT_SET_SCALE_W(agent,0.18,samus::FSMASH_LENGTH*0.06,0.18);
    }
}

#[acmd_script( agent = "samus", script = "effect_ice_lance_break", category = ACMD_EFFECT)]
unsafe fn effect_ice_lance_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_ice"),false,false);
        macros::EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("handr"), 7.5, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::AFTER_IMAGE_OFF(agent, 0);
    }
}

#[acmd_script( agent = "samus", script = "effect_ice_punch", category = ACMD_EFFECT)]
unsafe fn effect_ice_punch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("handr"), -1.5, 0.0, 0.0, 0, 0, 90, 0.15, true);
        LAST_EFFECT_SET_SCALE_W(agent,0.175,0.275,0.175);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("handr"), 2.5, 0.0, 0.0, 0, 0, -90, 0.15, true);
        LAST_EFFECT_SET_SCALE_W(agent,0.175,0.275,0.175);
    }
}
#[acmd_script( agent = "samus", script = "effect_ice_punch_break", category = ACMD_EFFECT)]
unsafe fn effect_ice_punch_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        println!("Punch break!");
        EFFECT_OFF_KIND(agent,Hash40::new("sys_ice"),false,false);
        macros::EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
}


#[acmd_script( agent = "samus", script = "sound_ice_break", category = ACMD_SOUND)]
unsafe fn sound_ice_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        effect_ice_lance,
        effect_ice_lance_break,
        effect_ice_punch,
        effect_ice_punch_break,

        sound_ice_break,
    );
}