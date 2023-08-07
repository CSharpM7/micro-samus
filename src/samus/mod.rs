mod agent;
mod acmd;
mod status;
mod frame;

#[smashline::installer]
pub fn install_dev() {
    acmd::install();
    status::install();
    agent::install();
    frame::install();
}

pub fn install() {
    
    #[cfg(not(feature = "dev"))]{
        acmd::install();
        agent::install();
        status::install();
        frame::install();
    }
    
    #[cfg(feature = "dev")]{
        install_dev();
    }
}

use crate::imports::imports_agent::*;
unsafe fn suit_effect(boma: *mut BattleObjectModuleAccessor, battle_object: *mut BattleObject) {
    let is_ice = VarModule::is_flag(battle_object, samus::instance::flag::ICE_MODE);
    if is_ice {
        MotionModule::add_motion_partial(boma, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, Hash40::new("visor"), 0.0,1.0, false, false, 0.0, true, true, false); 
        if ArticleModule::is_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN) {
            LinkModule::send_event_nodes(boma, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
        }
    }
    else{
        MotionModule::remove_motion_partial(boma, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, false);
    }
}