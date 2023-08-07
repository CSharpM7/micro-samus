use crate::imports::imports_agent::*;

#[status_script(agent = "samus_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn missile_homing_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let remaining_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_INT_LIFE);
    if remaining_life < 10 {
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("samusd_missile_homing"), false, true);
    }

    return original!(weapon);
}
#[status_script(agent = "samus_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn missile_homing_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_missile"), hash40("h_life"));
    WorkModule::set_int(weapon.module_accessor, life,*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    return original!(weapon);
}

pub fn install() {
    install_status_scripts!(
        missile_homing_exec,
        missile_homing_main
    );

}