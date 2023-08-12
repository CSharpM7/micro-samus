use crate::imports::imports_agent::*;
use super::crawl_shared::*;

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SQUAT_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn squat_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,true);
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SQUAT_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn squat_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,false);
}

unsafe extern "C" fn squat_main(fighter: &mut L2CFighterCommon, f: bool) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CRAWL);
    if f {
        fighter.status_SquatF();
        return fighter.main_shift(squat_f_main_loop)
    }
    else{
        fighter.status_SquatB();
        return fighter.main_shift(squat_b_main_loop)
    }
}
unsafe extern "C" fn squat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}
unsafe extern "C" fn squat_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    #[cfg(feature = "dev")]
    VarModule::countdown_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);

    fighter.status_SquatF_Main();
    squat_disable_terms(fighter);
    check_bomb_input(fighter);

    return 0.into();
}
unsafe extern "C" fn squat_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    #[cfg(feature = "dev")]
    VarModule::countdown_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);

    fighter.status_SquatB_Main();
    squat_disable_terms(fighter);
    check_bomb_input(fighter);

    return 0.into();
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SQUAT_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&prev)
    && !VarModule::is_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CRAWL)
    {
        VarModule::off_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CRAWL);
        return original!(fighter);
    }
    if !VarModule::is_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CRAWL) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
    }
    VarModule::on_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CRAWL);

    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("squat_n"), 3.0,0.0, 0.0);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    fighter.status_SquatWait_common(0xc0.into());
    fighter.main_shift(squat_wait_main_loop)
}
unsafe extern "C" fn squat_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait_Main();
    squat_disable_terms(fighter);
    check_bomb_input(fighter);

    return 0.into();
}

#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn bomb_g_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return morph_force_crawl(fighter);
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn speciallw_g_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return morph_force_crawl(fighter);
}
unsafe extern "C" fn morph_force_crawl(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait_Main();
    let frame = MotionModule::frame(fighter.module_accessor);
    if frame >= 30.0 && frame <= samus::UNMORPH_FRAME+1.0 {
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let stick_y_sensitivity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), Hash40::new_raw(0x10d088fec9).hash);
        if stick_y < stick_y_sensitivity {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
            VarModule::set_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, samus::BOMB_COOLDOWN_MAX - (frame as i32));
            VarModule::on_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CRAWL);
            ControlModule::clear_command(fighter.module_accessor, false);
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
        }
    }
    return 0.into();
}


pub fn install() {
    install_status_scripts!(
        squat_f_main,
        squat_b_main,
        squat_wait_main,

        speciallw_g_exec,
        bomb_g_exec,
    );

}