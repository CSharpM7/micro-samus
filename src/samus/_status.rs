use crate::imports::imports_agent::*;

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
    //fighter.main_shift(squat_main_loop)
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

    fighter.status_SquatWait_common(0xc0.into());
    fighter.main_shift(squat_wait_main_loop)
}
unsafe extern "C" fn squat_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait_Main();
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("squat_n"), 3.0,0.0, 0.0);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    squat_disable_terms(fighter);
    check_bomb_input(fighter);

    return 0.into();
}

unsafe extern "C" fn squat_disable_terms(fighter: &mut L2CFighterCommon) {

    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);

    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    //WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);

    ControlModule::reset_sub_stick(fighter.module_accessor);
}
unsafe extern "C" fn check_bomb_input(fighter: &mut L2CFighterCommon) {
    let can_spawn = VarModule::countdown_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW)
    && can_spawn {
        ControlModule::clear_command(fighter.module_accessor, false);
        let article = *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB;
        let maxbomb = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"),hash40("bomb_max_req"));
        if (ArticleModule::get_active_num(fighter.module_accessor, article) as i32) < maxbomb {
            ArticleModule::generate_article_enable(fighter.module_accessor, article, false, -1);
            ArticleModule::shoot_exist(fighter.module_accessor, article, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            VarModule::set_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, samus::BOMB_COOLDOWN_MAX);
        }
    }
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ST_INIT);

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    
    let is_ice = VarModule::is_flag(fighter.battle_object, samus::instance::flag::ICE_MODE);
    VarModule::set_flag(fighter.battle_object, samus::instance::flag::ICE_MODE, !is_ice);

    return 0.into();
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_g = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_lw_l")} else {Hash40::new("special_lw_r")};
    let motion_a = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_air_lw_l")} else {Hash40::new("special_lw_r")};
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {motion_g} else {motion_a};
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let motion_g = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_lw_l")} else {Hash40::new("special_lw_r")};
        let motion_a = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_air_lw_l")} else {Hash40::new("special_lw_r")};
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), true.into());

        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return 0.into();
    }
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        1.0,
        0.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.0
    );
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_ON_CHANGE_LR)]
unsafe fn special_lw_lr(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_g = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_lw_l")} else {Hash40::new("special_lw_r")};
    let motion_a = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_air_lw_l")} else {Hash40::new("special_lw_r")};
    fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), true.into());

    0.into()
}

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

#[common_status_script(status = FIGHTER_STATUS_KIND_SQUAT_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon21status_SquatWait_MainEv")]
unsafe fn status_squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&prev){
        VarModule::set_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);
        return call_original!(fighter);
    }
    fighter.status_SquatWait_common(0xc0.into());
    return fighter.main_shift(squat_wait_main_loop)
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

        /* 
        squat_f_pre,
        squat_b_pre,
        squat_f_main,
        squat_b_main,
        */
        
        squat_wait_main,

        special_lw_pre,
        special_lw_main,
        special_lw_lr,

        missile_homing_exec,
        missile_homing_main
    );
    skyline::nro::add_hook(nro_main).unwrap();

}