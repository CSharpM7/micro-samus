use crate::imports::imports_agent::*;

unsafe fn morphball_control_input(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor, status_kind: i32,situation_kind: i32, lr: f32, frame: f32) {
    //Spawn a bomb
    if (
        ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) || 
        ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW) || 
        ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || 
        ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
    )
    && VarModule::is_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT)
    {
        ControlModule::clear_command(boma, true);
        VarModule::off_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT);
        VarModule::set_int(fighter.battle_object, samus::instance::int::SPECIAL_LW_ROTATIONS,0); 
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
    }
    //Guard / Stick Cancel
    if (ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD)
    || ControlModule::get_stick_y(boma) >= 0.1
    || (KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() < 0.2 && ControlModule::get_stick_y(boma).abs() < 0.2))
    && VarModule::is_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT)
    {
        ControlModule::clear_command(boma, true);
        MotionModule::set_frame_sync_anim_cmd(boma, samus::UNMORPH_FRAME, true,true,false);
        MotionModule::set_rate(boma, 1.0);

        VarModule::on_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_EXIT);
        VarModule::off_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT);
    }
    //Jump
    if (
        ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP)
        //&& situation_kind == *SITUATION_KIND_GROUND 
        //&& status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW
    ){
        //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
        ControlModule::clear_command(boma, true);
        fighter.check_jump_cancel();
    }
}

unsafe fn morphball_control_movement(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor, lr: f32, frame: f32) {
    let stick_x = ControlModule::get_stick_x(boma);
    //MotionModule::set_rate(boma, stick_x.abs());
    if (stick_x.signum() != lr
    && stick_x.abs() > 0.2)
    {
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
    }
    /* 
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    if speed_x < 0.05 {
        MotionModule::set_rate(boma, 0.0);
    }
    else {
        MotionModule::set_rate(boma, (speed_x/2.0).min(1.0));
    }
    */
    let rots = VarModule::get_int(fighter.battle_object, samus::instance::int::SPECIAL_LW_ROTATIONS); 
    if frame >= samus::LOOP_FRAME {
        if rots == 0 && !VarModule::is_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT){
            VarModule::on_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT);
        }
        VarModule::inc_int(fighter.battle_object, samus::instance::int::SPECIAL_LW_ROTATIONS); 
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), samus::MORPH_FRAME, 1.0, 1.0);
    }

    let canAttack = if MotionModule::motion_kind(boma) == Hash40::new("special_lw").hash
    {
        (rots > 0 || frame >= 25.0)
    }
    else{
        VarModule::is_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT)
    };
}

unsafe fn morphball_moveset(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor, status_kind: i32,situation_kind: i32, lr: f32, frame: f32) {    
    if [
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, 
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, 
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, 
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, 
    ].contains(&status_kind) {
        if [
            *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, 
            *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, 
            *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, 
        ].contains(&status_kind) {
            VarModule::off_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT);
            VarModule::off_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_EXIT); 
            if frame <= 1.0 {
                VarModule::set_int(fighter.battle_object, samus::instance::int::SPECIAL_LW_ROTATIONS,0); 
            }
            else if (frame >= samus::MORPH_FRAME-1.0){
                let new_status = if situation_kind == *SITUATION_KIND_GROUND {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW} else {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW};
                StatusModule::change_status_request_from_script(boma,new_status, true);
            }
        }
        let isBall = (frame >= samus::MORPH_FRAME && frame < samus::UNMORPH_FRAME);
        //Reset variables
        if !isBall  {
            if frame <= 10.0{
                VarModule::off_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CAN_EXIT);
                VarModule::off_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_EXIT); 
                VarModule::set_int(fighter.battle_object, samus::instance::int::SPECIAL_LW_ROTATIONS,0); 
            }
            else if !VarModule::is_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_EXIT){
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), samus::MORPH_FRAME, 1.0, 1.0);
            }
        }
        else {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            morphball_control_movement(fighter,boma,lr,frame);
            morphball_control_input(fighter,boma,status_kind,situation_kind,lr,frame);
        }
    }
}


unsafe fn specials_force_weapon(boma: *mut BattleObjectModuleAccessor, battle_object: *mut BattleObject, status_kind: i32,situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if VarModule::is_flag(battle_object, samus::instance::flag::ICE_MODE){
        if (&[
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
        ]).contains(&status_kind) 
        && frame < 6.0 {
            StatusModule::change_status_request_from_script(boma,
                if situation_kind == *SITUATION_KIND_AIR{*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A} else {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G},
            false);
        }
    }
    else {
        if (&[
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G,
        ]).contains(&status_kind) 
        && frame < 6.0 {
            StatusModule::change_status_request_from_script(boma,
                if situation_kind == *SITUATION_KIND_AIR{*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A} else {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G},
            false);
        }
    }
}


unsafe fn samus_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    VarModule::countdown_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let frame = MotionModule::frame(boma);
    let lr = PostureModule::lr(boma);

    let fx_fix_rate = 15.0;
    let modulo = frame % fx_fix_rate;
    if modulo < 1.0 || frame <= 1.0 {
        super::suit_effect(boma,fighter.battle_object);
    }
    VarModule::countdown_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, 0);
       
    //super::suit_effect(fighter.module_accessor, fighter.battle_object);
    //morphball_moveset(fighter,boma,status_kind,situation_kind,lr,frame);
    specials_force_weapon(boma,fighter.battle_object,status_kind,situation_kind,frame);
    /* 
    if (&[
        *FIGHTER_STATUS_KIND_SQUAT_F,
        *FIGHTER_STATUS_KIND_SQUAT_B
    ]).contains(&status_kind) {
        let speed_mul = if status_kind == *FIGHTER_STATUS_KIND_SQUAT_F {1.25} else {1.5};
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
        //println!("Force crawl");
        //StatusModule::change_status_force(boma, FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW.into(), false.into());
    }*/
}

#[fighter_frame( agent = FIGHTER_KIND_SAMUS )]
fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        samus_update(fighter);
    }
}
#[smashline::fighter_frame_callback]
fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let category = smash::app::utility::get_category(boma);
        let kind = smash::app::utility::get_kind(boma);
        if category == BATTLE_OBJECT_CATEGORY_FIGHTER && kind == FIGHTER_KIND_SAMUS {
            samus_update(fighter);
        }
    }
}
#[smashline::weapon_frame_callback]
pub fn missile_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_SAMUS_MISSILE {
            return
        }
        let status = StatusModule::status_kind(weapon.module_accessor);
        let homing = *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING;
        println!("Status: {status} Homing: {homing}");
    }
}

pub fn install() {
    #[cfg(feature = "dev")]
    smashline::install_agent_frame_callbacks!(
      global_fighter_frame
    );
    #[cfg(not(feature = "dev"))]
    smashline::install_agent_frames!(
        samus_frame
    );
}