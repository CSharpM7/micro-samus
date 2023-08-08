use crate::imports::imports_acmd::*;

#[acmd_script( agent = "samus_missile", script = "game_homing", category = ACMD_GAME, low_priority )]
unsafe fn game_homing(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 65, 25, 0, 36, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}
#[acmd_script( agent = "samus_missile", script = "effect_homing", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_homing(agent: &mut L2CAgentBase) {
    macros::EFFECT_FOLLOW(agent, Hash40::new("samus_missile_homing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, true);
    LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
    for i in 1..i32::MAX{
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, true);
            LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
        }
        wait(agent.lua_state_agent, 15.0);
    }
}

#[acmd_script( agent = "samus_missile", script = "game_hburst", category = ACMD_GAME, low_priority )]
unsafe fn game_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}
#[acmd_script( agent = "samus_missile", script = "effect_hburst", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "samus_missile", script = "sound_hburst", category = ACMD_SOUND, low_priority )]
unsafe fn sound_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_homing,
        effect_homing,

        game_hburst,
        effect_hburst,
        sound_hburst,
    );
}