use crate::imports::imports_agent::*;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    super::suit_effect(boma, fighter.battle_object);
    if is_damage_status(boma){
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
        if VarModule::is_flag(fighter.battle_object, samus::status::flag::ATTACK_HAS_ICE){
            //MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_ice_break"), -1);
            VarModule::off_flag(fighter.battle_object, samus::status::flag::ATTACK_HAS_ICE);
        }
    }
    true.into()
}

pub unsafe fn agent_rebirth(fighter: &mut L2CFighterCommon){
    
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    agent_rebirth(fighter);
    return original!(fighter);
}

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_SAMUS {
        return;
    }
    agent_rebirth(fighter);
    GetVarManager::reset_var_module(fighter.battle_object,false);
    VarModule::set_flag(fighter.battle_object, samus::instance::flag::ICE_MODE, false);
    VarModule::set_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}


#[smashline::fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
    install_status_scripts!(
        rebirth_pre,
    );
}