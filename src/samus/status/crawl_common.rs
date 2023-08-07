use crate::imports::imports_agent::*;
use super::crawl_shared::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_squat_walk_uniq_process_main)]
pub unsafe fn status_sub_squat_walk_main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    squat_disable_terms(fighter);
    return call_original!(fighter);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_SquatWait_Main)]
pub unsafe fn status_sub_squat_wait_main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    squat_disable_terms(fighter);
    return call_original!(fighter);
}

#[common_status_script(status = FIGHTER_STATUS_KIND_SQUAT_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,*FIGHTER_STATUS_KIND_SQUAT_WAIT,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&prev){
        VarModule::set_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);
        return call_original!(fighter);
    }
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("squat_n"), 3.0,0.0, 0.0);
    fighter.status_SquatWait_common(0xc0.into());
    return fighter.main_shift(squat_wait_main_loop)
}

unsafe extern "C" fn squat_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    squat_disable_terms(fighter);
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if motion == Hash40::new("squat_n").hash{
        fighter.status_SquatWait_Main();
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
            MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("squat_attack"), 0.0,0.0, 0.0);
        }
    }
    else{
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        if MotionModule::is_end(fighter.module_accessor){
            StatusModule::change_status_force(fighter.module_accessor, FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            //MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("squat_n"), 3.0,0.0, 0.0);
        }
    }
    //let current_rate = MotionModule::rate(fighter.module_accessor);
    //let new_rate = if current_rate <= 0.1 {0.0} else {current_rate/2.0};
    //MotionModule::set_rate(fighter.module_accessor, new_rate);
    //squat_disable_terms(fighter);
    //check_bomb_input(fighter);

    return 0.into();
}

fn nro_main(nro: &skyline::nro::NroInfo) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                status_sub_squat_wait_main_hook,
                status_sub_squat_walk_main_hook
            );
        }
        _ => (),
    }
}
pub fn install() {
    install_status_scripts!(
        status_squat_wait_main,     
    );
    skyline::nro::add_hook(nro_main).unwrap();

}