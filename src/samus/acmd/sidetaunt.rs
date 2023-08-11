use crate::imports::imports_acmd::*;

#[acmd_script( agent = "samus", scripts = ["effect_appealsl","effect_appealsr"], category = ACMD_EFFECT)]
unsafe fn effect_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 7.046, 0.555, -0.197, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 37.0);
}
#[acmd_script( agent = "samus", scripts = ["sound_appealsl","sound_appealsr"], category = ACMD_SOUND)]
unsafe fn sound_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_swing_s"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_squat"));
    }
    //frame(agent.lua_state_agent, 52.0);
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_rise"));
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_appealsl","expression_appealsr"], category = ACMD_EXPRESSION)]
unsafe fn expression_appeals(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        effect_appeals,
        sound_appeals,
        expression_appeals,
    );
}