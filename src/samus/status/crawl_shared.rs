use crate::imports::imports_agent::*;

pub unsafe extern "C" fn squat_disable_terms(fighter: &mut L2CFighterCommon) {

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
    ControlModule::reset_sub_stick_y(fighter.module_accessor);
}

pub unsafe extern "C" fn check_bomb_input(fighter: &mut L2CFighterCommon) {
    let can_spawn = VarModule::countdown_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
        let cooldown = VarModule::get_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN);
        ControlModule::clear_command(fighter.module_accessor, false);
        let article = *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB;
        let maxbomb = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"),hash40("bomb_max_req"));
        if (ArticleModule::get_active_num(fighter.module_accessor, article) as i32) < maxbomb && can_spawn {
            ArticleModule::generate_article_enable(fighter.module_accessor, article, false, -1);
            ArticleModule::shoot_exist(fighter.module_accessor, article, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            VarModule::set_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, samus::BOMB_COOLDOWN_MAX);
        }
    }
}
