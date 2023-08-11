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

#[acmd_script( agent = "samus_cshot", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn game_shoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 42, 0, 14, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 28.0, 40, 50, 0, 46, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
    }
}

#[acmd_script( agent = "samus_cshot", script = "sound_shoot", category = ACMD_SOUND, low_priority )]
unsafe fn sound_shoot(agent: &mut L2CAgentBase) {
    /* 
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_samus_special_n01"));
    }
    WorkModule::get_float(agent.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    if(methodlib::L2CValue::operator<=(lib::L2CValueconst&)const(256215000, 0.25)){
        if macros::is_excute(agent) {
            macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n02"));
        }
        else{
        WorkModule::get_float(agent.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        if(methodlib::L2CValue::operator<=(lib::L2CValueconst&)const(256215000, 0.625)){
            if macros::is_excute(agent) {
                macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n03"));
            }
            else{
            WorkModule::get_float(agent.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
            if(methodlib::L2CValue::operator<=(lib::L2CValueconst&)const(256215000, 0.875)){
                if macros::is_excute(agent) {
                    macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n04"));
                }
                else{
                if macros::is_excute(agent) {
                    macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n05"));
                }
            }
        }
    }
}
}
}*/
}

#[acmd_script( agent = "samus_cshot", script = "game_ice", category = ACMD_GAME)]
unsafe fn game_ice(agent: &mut L2CAgentBase) {
    println!("Pew in game");
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_ENERGY);

    }
}
#[acmd_script( agent = "samus_cshot", script = "sound_ice", category = ACMD_SOUND)]
unsafe fn sound_ice(agent: &mut L2CAgentBase) {
    println!("Pew in sound");
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_samus_special_n01"));
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n02"));
    }
}
#[acmd_script( agent = "samus_cshot", script = "effect_ice", category = ACMD_EFFECT)]
unsafe fn effect_ice(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 0, 0, 225, 0, 0, 0.7, true);

        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_specialnrapid,
        game_ice,
        sound_ice,
        effect_ice,

        game_shoot,
        sound_shoot,
    );
}