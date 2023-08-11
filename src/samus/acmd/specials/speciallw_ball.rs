use crate::imports::imports_acmd::*;

#[acmd_script( agent = "samus", scripts = ["game_speciallw","game_specialairlw"], category = ACMD_GAME)]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        //WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV_CONT);
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
        }
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
    //FT_MOTION_RATE(agent, 0.6);
}

#[acmd_script( agent = "samus", scripts = ["effect_speciallw","effect_specialairlw"], category = ACMD_EFFECT)]
unsafe fn effect_speciallw(fighter: &mut L2CAgentBase) {
    //frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("special_lw").hash{
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT) {
            //macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_bomb_jump"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.48, true);
        }
    }
    frame(fighter.lua_state_agent, samus::UNMORPH_FRAME+5.0);
    if macros::is_excute(fighter) {
    }

}

#[acmd_script( agent = "samus", scripts = ["sound_speciallw","sound_specialairlw"], category = ACMD_SOUND)]
unsafe fn sound_speciallw(fighter: &mut L2CAgentBase) {
    //frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
    }
}

#[acmd_script( agent = "samus", scripts = ["expression_speciallw","expression_specialairlw"], category = ACMD_EXPRESSION)]
unsafe fn expression_speciallw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, samus::MORPH_FRAME);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true,0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_speciallw,
        effect_speciallw,
        sound_speciallw,
        expression_speciallw,
    );
}