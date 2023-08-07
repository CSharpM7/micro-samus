use crate::imports::imports_agent::*;
use super::crawl_shared::*;

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SQUAT_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn squat_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_pre(fighter);
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SQUAT_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn squat_b_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_pre(fighter);
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SQUAT_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn squat_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,true);
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SQUAT_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn squat_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,false);
}

unsafe extern "C" fn squat_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ST_INIT);

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV_CONT);
    return 1.into();
}
unsafe extern "C" fn squat_main(fighter: &mut L2CFighterCommon, f: bool) -> L2CValue {
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
    fighter.status_SquatF_Main();
    squat_disable_terms(fighter);
    check_bomb_input(fighter);

    return 0.into();
}
unsafe extern "C" fn squat_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    ].contains(&prev){
        VarModule::set_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);
        return original!(fighter);
    }

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

pub fn install() {
    install_status_scripts!(
        //squat_f_pre,
        //squat_b_pre,

        squat_f_main,
        squat_b_main,

        squat_wait_main
    );

}