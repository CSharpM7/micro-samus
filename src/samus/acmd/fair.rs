use crate::imports::imports_acmd::*;

pub const DAMAGE: f32 = 14.0;
pub const ANGLE: u64 = 46;
pub const KBG: i32 = 100;
pub const BKB: i32 = 30;

#[acmd_script( agent = "samus", script = "game_attackairf2", category = ACMD_GAME)]
unsafe fn game_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), DAMAGE, ANGLE, KBG, 0, BKB, 3.0, 3.75, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), DAMAGE, ANGLE, KBG, 0, BKB, 3.0, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "samus", script = "game_attackairf2_break", category = ACMD_GAME)]
unsafe fn game_attackairf_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_power(agent.module_accessor, 0, DAMAGE-2.0, false);
        AttackModule::set_power(agent.module_accessor, 1, DAMAGE-2.0, false);
    }
}

#[acmd_script( agent = "samus", script = "effect_attackairf2", category = ACMD_EFFECT)]
unsafe fn effect_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        MotionAnimcmdModule::call_script_single(agent.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_ice_punch"), -1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 10, 4, 0, -7, -103, 1.25, true, *EF_FLIP_YZ);

        LAST_EFFECT_SET_COLOR(agent,0.25, 0.75,1.0);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_ice"),false,false);
    }
}
#[acmd_script( agent = "samus", script = "sound_attackairf2", category = ACMD_SOUND)]
unsafe fn sound_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_swing_l"));
    }
}
#[acmd_script( agent = "samus", script = "expression_attackairf2", category = ACMD_EXPRESSION)]
unsafe fn expression_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attacks"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


pub fn install() {
    install_acmd_scripts!(
        game_attackairf,
        game_attackairf_break,

        effect_attackairf,
        sound_attackairf,
        expression_attackairf,
    );
}