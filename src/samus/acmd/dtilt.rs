use crate::imports::imports_acmd::*;

pub const DAMAGE: f32 = 10.0;
pub const ANGLE: u64 = 85 ;
pub const KBG: i32 = 50;
pub const BKB: i32 = 80;
pub const SIZE: f32 = 5.8;

#[acmd_script( agent = "samus", script = "game_attacklw32", category = ACMD_GAME)]
unsafe fn game_attacklw3(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 8.0, ANGLE, KBG, 0, BKB, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_BOMB);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);

        if VarModule::is_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
            macros::ATTACK(agent, 0, 0, Hash40::new("throw"), DAMAGE, ANGLE, KBG, 0, BKB, SIZE, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);
        }
        else{
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, ANGLE, KBG, 0, BKB, SIZE, 0.0, 1.6, 14.4, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_BOMB);
        }
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        if VarModule::is_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
            macros::ATTACK(agent, 2, 0, Hash40::new("throw"), DAMAGE, ANGLE, KBG, 0, BKB, SIZE, 0.0, -8.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);
        }
        else{
            AttackModule::clear(agent.module_accessor, 0, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
            macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 8.0, 90, KBG, 0, 40, SIZE, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);

            macros::ATTACK(agent, 2, 0, Hash40::new("throw"), 8.0, 90, KBG, 0, 40, SIZE, 0.0, -8.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);

            //macros::ATTACK(agent, 3, 0, Hash40::new("throw"), 8.0, 90, KBG, 0, 40, SIZE, 0.0, -16.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);
        }
    }
    frame(agent.lua_state_agent, 12.0);
    FT_MOTION_RATE_RANGE(agent,12.0,20.0,21.0);
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    FT_MOTION_RATE(agent,1.0);
}

#[acmd_script( agent = "samus", script = "effect_attacklw32", category = ACMD_EFFECT)]
unsafe fn effect_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("throw"), 0, 0, 0, 225, 0, 0, 0.7, true);
            macros::EFFECT(agent, Hash40::new("sys_ice_landing"), Hash40::new("top"), 0, 0, 15.0, 0, 0, 0, 1.125, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            macros::EFFECT(agent, Hash40::new("sys_steam"), Hash40::new("top"), 0, 1.25, 13.4, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent,0.375,0.75,1.0);
            macros::EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("throw"), 0.0, -8.0, 0.0, 225, 0, 0, 0.7, true);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("throw"), 0.0, -16.0, 0.0, 225, 0, 0, 0.7, true);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
            EFFECT_DETACH_KIND(agent,Hash40::new("sys_ice"),-1);
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
            MotionAnimcmdModule::call_script_single(agent.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_attacklw32end"), -1);
            MotionAnimcmdModule::call_script_single(agent.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_ice_break"), -1);
            VarModule::off_flag(agent.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR);
        }
    }
}
#[acmd_script( agent = "samus", script = "effect_attacklw32end", category = ACMD_EFFECT)]
unsafe fn effect_attacklw3_end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_ice"),false,false);

        macros::EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("throw"), 0.0, -4.0, 0.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("throw"), 0.0, -12.0, 0.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}


#[acmd_script( agent = "samus", script = "sound_attacklw32", category = ACMD_SOUND)]
unsafe fn sound_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_freeze"));
    }
}

#[acmd_script( agent = "samus", script = "expression_attacklw32", category = ACMD_EXPRESSION)]
unsafe fn expression_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_attacklw3,
        effect_attacklw3,
        sound_attacklw3,
        expression_attacklw3,

        effect_attacklw3_end,
    );
}