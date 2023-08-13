use crate::imports::imports_agent::*;
use crate::samus::*;

/* 
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));

    let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);

    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {Hash40::new("special_n_r")} else {Hash40::new("special_air_n_r")};
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    //ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    special_n_shot_change(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}


unsafe extern "C" fn special_n_shot_change(fighter: &mut L2CFighterCommon) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT) {
        let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        WorkModule::set_customize_no(article_boma, 1, 0);
        WorkModule::set_float(article_boma, -1.0, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    }
}
unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_r").into(), Hash40::new("special_air_n_r").into(),true.into());

        let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP} else {*GROUND_CORRECT_KIND_AIR};
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));
    
        let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART)
    || MotionModule::is_end(fighter.module_accessor){
        if ControlModule::check_button_on(fighter.module_accessor, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER)
        || ControlModule::check_button_on(fighter.module_accessor, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
            let motion = MotionModule::motion_kind(fighter.module_accessor);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
            special_n_shot_change(fighter);
        }
        else{
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(),true.into());
        }
    }
    
    if ControlModule::check_button_trigger(fighter.module_accessor, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER)
    || ControlModule::check_button_trigger(fighter.module_accessor, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
        let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {Hash40::new("special_n_f")} else {Hash40::new("special_air_n_f")};
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(),true.into());
    }
    //This is the part where you do is_enable_transition_term on everything
    if WorkModule::is_enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON){
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(),true.into());
        //FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C for not JC
    }
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
    
    0.into()
}

    */
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }

    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    let c = charge as f32 / charge_max as f32;
    //let is_max = charge >= charge_max;
    let is_max = false;
    let motion_g = if !is_max {Hash40::new("special_n_i")} else {Hash40::new("special_n_i")};
    let motion_a = if !is_max {Hash40::new("special_air_n_i")} else {Hash40::new("special_air_n_i")};
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {motion_g} else {motion_a};

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART){
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
    }
    else{
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0,false, false);
    }

    let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));

    let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    
    //WorkModule::set_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT,0);
    //Something about slopes?
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_f_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
        let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
        //let is_max = charge >= charge_max;
        let is_max = false;

        let motion_g = if !is_max {Hash40::new("special_n_i")} else {Hash40::new("special_n_i")};
        let motion_a = if !is_max {Hash40::new("special_air_n_i")} else {Hash40::new("special_air_n_i")};
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(),true.into());

        let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));
    
        let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    0.into()
}


#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_f_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
    return original!(fighter);
}

pub fn install() {
    install_status_scripts!(
        special_n_f_main,
        special_n_f_end,
    );
}