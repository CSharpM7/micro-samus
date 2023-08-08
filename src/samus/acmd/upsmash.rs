use crate::imports::imports_acmd::*;

pub const DAMAGE: f32 = 10.0;
pub const ANGLE: u64 = 85 ;
pub const KBG: i32 = 50;
pub const BKB: i32 = 80;
pub const SIZE: f32 = 6.8;


#[acmd_script( agent = "samus", script = "game_attackhi42", category = ACMD_GAME, low_priority )]
unsafe fn game_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 12.0, 361, 112, 0, 16, 2.8, -1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 14.0, 361, 100, 0, 40, 3.5, 3.75, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 14.0, 361, 100, 0, 40, 3.25, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("armr"), 14.0, 361, 100, 0, 40, 3.0, samus::FSMASH_LENGTH-1.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "samus", script = "effect_attackhi42", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.5);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("handr"), 5.0, 0.0, 0.0, 0, 0, -90, 0.2, true);
        LAST_EFFECT_SET_SCALE_W(agent,0.18,samus::FSMASH_LENGTH*0.06,0.18);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        //macros::EFFECT(agent, Hash40::new("sys_ice_hit"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);

        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_samus_ice1"), Hash40::new("tex_samus_ice1"), 4, 
        Hash40::new("armr"), 3.0, 0.0, 0.0, Hash40::new("armr"), 17.0, 0.0, 0.0, 
        true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
        1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_ice"),false,false);
    }
}


#[acmd_script( agent = "samus", script = "sound_attackhi42", category = ACMD_SOUND, low_priority )]
unsafe fn sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_sword_swing_l"));
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
    }
}

#[acmd_script( agent = "samus", script = "expression_attackhi42", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        //ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"), false, -1.0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("s4s2"), false, -1.0);
        LinkModule::send_event_nodes(agent.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
}

#[acmd_script( agent = "samus", script = "sound_attackhi4charge2", category = ACMD_SOUND, low_priority )]
unsafe fn sound_attackhi4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("vc_samus_attack06"));
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
}

#[acmd_script( agent = "samus", script = "effect_attackhi4charge2", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackhi4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for i in 1..i32::MAX{
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
            
            macros::EFFECT(agent, Hash40::new("sys_starrod_splash"), Hash40::new("armr"), 8.0, 0.0, 0.0, 0, 0, -90, 1, 4, 4, 4, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
    }
}
#[acmd_script( agent = "samus", script = "expression_attackhi4charge2", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_attackhi4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        //if !ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN)
        {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
            //ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("s4s2"), false, -1.0);
            //physics!(*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.9, 0.8, -1, 0.9, 0.8, -1, Hash40::new("invalid"));
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
            LinkModule::send_event_nodes(agent.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
        }

        println!("Charge expression");
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_attackhi4,
        effect_attackhi4,
        sound_attackhi4,
        expression_attackhi4,

        sound_attackhi4charge,
        effect_attackhi4charge,
        expression_attackhi4charge,
    );
}