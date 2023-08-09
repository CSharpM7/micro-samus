use crate::imports::imports_acmd::*;

#[acmd_script( agent = "samus", scripts = ["game_squatf","game_squatb","game_squatn"], category = ACMD_GAME, low_priority )]
unsafe fn game_crawl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    }
    /* 
    for i in 1..i32::MAX{
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        }
        wait(fighter.lua_state_agent, 1.0);
    }*/
}

#[acmd_script( agent = "samus", scripts = ["effect_squatf","effect_squatb"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_crawl(fighter: &mut L2CAgentBase) {
    //frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("special_lw").hash{
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script( agent = "samus", scripts = ["sound_squatf","sound_squatb"], category = ACMD_SOUND, low_priority )]
unsafe fn sound_crawl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        let prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        if ![
            *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW,*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW
        ].contains(&prev){
            macros::PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
        }
    }
}

#[acmd_script( agent = "samus", scripts = ["expression_squatf","expression_squatb"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_crawl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10, true);
    }
    for i in 1..i32::MAX{
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 30.0);
    }
}

#[acmd_script( agent = "samus", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
}
#[acmd_script( agent = "samus", script = "effect_speciallw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        //macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_crawl,
        effect_crawl,
        sound_crawl,
        expression_crawl,

        game_speciallw,
        effect_speciallw,
    );
}