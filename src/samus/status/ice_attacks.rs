use crate::imports::imports_agent::*;
use crate::samus::*;

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    fighter.status_AttackLw3_common();
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw32"), 0.0, 1.0, false, 0.0, false, false);
    let groundcheck = *GROUND_TOUCH_FLAG_DOWN as u32;
    let lr = PostureModule::lr(fighter.module_accessor);
    let nearGround = GroundModule::ray_check(
        fighter.module_accessor, 
        &smash::phx::Vector2f{ x: PostureModule::pos_x(fighter.module_accessor) + (14.5 * lr), y: PostureModule::pos_y(fighter.module_accessor)}, 
        &Vector2f{ x: 0.0, y: -2.0}, true
    ) == 1;
    println!("Near Ground: {nearGround}");
    VarModule::set_flag(fighter.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR, nearGround);
    fighter.sub_shift_status_main(L2CValue::Ptr(attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackLw3_Main();
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn attacklw3_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, samus::status::flag::ATTACK_LW3_ICE_PILLAR) {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_attacklw32end"), -1);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_attacklw32end"), -1);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        attacklw3_main,
        attacklw3_exit     
    );
}