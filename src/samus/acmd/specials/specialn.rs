use crate::imports::imports_acmd::*;

#[acmd_script( agent = "samus", script = "game_specialnrapid", category = ACMD_GAME)]
unsafe fn game_specialnrapid(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT) {
            let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);
            WorkModule::set_customize_no(article_boma, 1, 0);

            ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            WorkModule::set_float(article_boma, -1.0, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_specialnrapid,
    );
}