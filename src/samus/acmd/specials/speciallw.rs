use crate::imports::imports_acmd::*;

#[acmd_script( agent = "samus", scripts = ["game_speciallwl","game_specialairlwl","game_speciallwr","game_specialairlwr"], category = ACMD_GAME)]
unsafe fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent,0.425);
}

#[acmd_script( agent = "samus", scripts = ["effect_speciallwl","effect_specialairlwl","effect_speciallwr","effect_specialairlwr"], category = ACMD_EFFECT)]
unsafe fn effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    let is_ice = VarModule::is_flag(agent.battle_object, samus::instance::flag::ICE_MODE);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_appeal_s"), Hash40::new("armr"), 7, 0, 0, 0, 0, 90, 1, true);
        LAST_EFFECT_SET_RATE(agent,2.25);
        if is_ice{
            LAST_EFFECT_SET_COLOR(agent,0.0, 0.875,1.25);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        //let is_ice = VarModule::is_flag(agent.battle_object, samus::instance::flag::ICE_MODE);
        if is_ice{
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("armr"), 7, 0, 0, 0, 0, 90, 1, true);
            LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);

            macros::EFFECT_FOLLOW(agent,Hash40::new("sys_hit_ice"), Hash40::new("armr"), 8, 0, 0, 0, 0, 90, 0.2, true);
        }
    }
    frame(agent.lua_state_agent, 85.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_sscope_bullet"),false,false);
    }
}

#[acmd_script( agent = "samus", scripts = ["sound_speciallwl","sound_specialairlwl","sound_speciallwr","sound_specialairlwr"], category = ACMD_SOUND)]
unsafe fn sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_appeal_s01"));
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        let is_ice = VarModule::is_flag(agent.battle_object, samus::instance::flag::ICE_MODE);
        if !is_ice{
            macros::PLAY_SE(agent, Hash40::new("se_samus_appeal_s03"));
        }
        else{
            macros::PLAY_SE(agent, Hash40::new("se_common_frieze_ll"));
        }
    }
    frame(agent.lua_state_agent, 85.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_samus_appeal_s03"));
        macros::STOP_SE(agent, Hash40::new("se_common_frieze_ll"));
        macros::PLAY_SE(agent, Hash40::new("se_samus_appeal_s04"));
    }
}

#[acmd_script( agent = "samus", scripts = ["expression_speciallwl","expression_specialairlwl","expression_speciallwr","expression_specialairlwr"], category = ACMD_EXPRESSION)]
unsafe fn expression_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("appeal_sl"), false, -1.0);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 2.25);
    }
    frame(agent.lua_state_agent, 36.0);
    let is_ice = VarModule::is_flag(agent.battle_object, samus::instance::flag::ICE_MODE);
    if macros::is_excute(agent) {
        let rumble = if is_ice {Hash40::new("rbkind_15_iceberg_sp")} else {Hash40::new("rbkind_elecattacks")};
        ControlModule::set_rumble(agent.module_accessor, rumble, 40, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 168.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
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