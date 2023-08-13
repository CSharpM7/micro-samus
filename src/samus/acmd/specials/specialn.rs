use crate::imports::imports_acmd::*;

#[acmd_script( agent = "samus", script = "game_specialnrapid", category = ACMD_GAME)]
unsafe fn game_specialnrapid(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT) {
            let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);
            WorkModule::set_customize_no(article_boma, 1, 0);

            ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            WorkModule::set_float(article_boma, -1.0, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
        }
    }
}
#[acmd_script( agent = "samus", scripts = ["game_specialnstart","game_specialairnstart"], category = ACMD_GAME, low_priority )]
unsafe fn game_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, samus::instance::flag::ICE_MODE) {
            ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_BULLET_DISP);
        }
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_specialnhold","effect_specialairnhold"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialnhold(agent: &mut L2CAgentBase) {
    let mut is_ice = false;
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        is_ice = VarModule::is_flag(agent.battle_object, samus::instance::flag::ICE_MODE);
        if !is_ice {
            EFFECT_FOLLOW(agent, Hash40::new("samus_cshot_hold"), Hash40::new("armr"), 7.98, -0.506, -0.251, -91.273, -1.797, 176.373, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        else{
            EFFECT_FOLLOW(agent, Hash40::new("samus_cshot_hold"), Hash40::new("armr"), 7.98, -0.506, -0.251, -91.273, -1.797, 176.373, 0.75, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            LAST_EFFECT_SET_COLOR(agent,0.25, 1.5,1.0);

            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("armr"), 7.0, 0.0, 0.0, 0, 0, -90, 0.075, true);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    for i in 0..100 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 12, 0, 12, 0, 0, 0, false);
            if is_ice {
                //let size = 1.0+(i as f32 / 6.0);
                //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("armr"), 7.5, 0, 0, 0, 0, 0, size, true);
                //LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
            }
        }
        wait(agent.lua_state_agent, 12.0);
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_specialnhold","sound_specialairnhold"], category = ACMD_SOUND, low_priority )]
unsafe fn sound_specialnhold(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, samus::instance::flag::ICE_MODE) {
            macros::PLAY_STATUS(agent, Hash40::new("se_samus_special_n01"));
        }
        else{
            macros::PLAY_STATUS(agent, Hash40::new("se_samus_special_n01"));
        }
    }
}

#[acmd_script( agent = "samus", scripts = ["game_specialnice","game_specialairnice"], category = ACMD_GAME, low_priority )]
unsafe fn game_specialnice(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_specialnice","effect_specialairnice"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialnice(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 6, 6, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::FLASH(agent, 1, 0.753, 1, 0.706);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 10, 0.314, 0.314, 0.314, 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_specialnice","sound_specialairnice"], category = ACMD_SOUND, low_priority )]
unsafe fn sound_specialnice(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_samus_special_n01"));
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let charge = WorkModule::get_int(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
        let charge_max = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame"));
        let c = charge / charge_max;
        println!("Charge: {charge} Ratio: {c}");
        if c <= 0.25 {
            macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n02"));
        }
        else if c <= 0.625 {
            macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n03"));
        }
        else if c <= 0.875 {
            macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n04"));
        }
        else {
            macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n05"));
        }
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_specialnice","expression_specialairnice"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_specialnice(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_specialnrapid,

        game_specialnstart,
        effect_specialnhold,
        sound_specialnhold,

        game_specialnice,
        effect_specialnice,
        sound_specialnice,
        expression_specialnice,
    );
}