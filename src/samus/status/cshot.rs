use crate::imports::imports_agent::*;
use crate::samus::*;

#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn cshot_shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = get_battle_object_from_id(owner_id);
    let mut charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let eff = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    println!("Shoot Init charge: {charge} Eff: {eff}");
    let is_ice = charge < 0.0 || eff <= 0;
    if !is_ice {
        return original!(weapon);
    };
    charge = charge.abs();
    WorkModule::set_float(weapon.module_accessor, -charge,*WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let c = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    println!("ICE PEW. Charge: {c}");

    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cshot"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            let min_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_scale"));
            let max_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_scale"));
        
            let scale = min_scale + (max_scale-min_scale)*charge;
            effect!(
                weapon,
                MA_MSC_EFFECT_REQUEST_FOLLOW,
                Hash40::new("samus_cshot_bullet"),
                Hash40::new("top"),
                7.98004,
                -0.50584,
                -0.25092,
                -91.2728,
                -1.7974,
                176.373,
                scale,
                false,
                0,
                0,
                0
            );
            weapon.clear_lua_stack();
            lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
            sv_module_access::effect(weapon.lua_state_agent);
            let handle = weapon.pop_lua_stack(1).get_i32();
            //WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
        }
    }
    let lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_SHOOT_LR);
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("angle"));
    let min_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_speed"));
    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_speed"));

    let speed = min_speed + (max_speed-min_speed)*charge;

    let speed_x = angle.to_radians().cos() * speed * lr;
    let speed_y = angle.to_radians().sin() * speed;
    weapon.clear_lua_stack();
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.0
    );
    if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let min_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_scale"));
        let max_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_scale"));
        let scale = min_scale + (max_scale-min_scale)*charge;
        /*
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samus_cshot_bullet_sub_b"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            scale,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
 */
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samus_cshot_bullet_sub"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            scale,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW_SUB);
    }
    0.into()
}

#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn cshot_shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let eff = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    let is_ice = charge < 0.0 || eff <= 0;
    println!("Eff Main: {eff}");
    if !is_ice {
        return original!(weapon);
    }
    if charge > 0.0{
        WorkModule::set_float(weapon.module_accessor, -charge,*WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    }
    GroundModule::set_passable_check(weapon.module_accessor, true);
    WorkModule::set_customize_no(weapon.module_accessor, 1, 0);
    WorkModule::set_float(weapon.module_accessor, -1.0,*WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    
    cshot_shoot_ice(weapon);
/* 
    let param_life =  WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("life")) as i32;
    WorkModule::set_int(weapon.module_accessor, param_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, param_life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

        AttackModule::set_lerp_ratio(weapon.module_accessor, charge, 0);
    */
    weapon.fastshift(L2CValue::Ptr(cshot_shoot_main_shift as *const () as _));

    return 0.into()
}
unsafe extern "C" fn cshot_shoot_ice(weapon: &mut L2CWeaponCommon) {
    EFFECT_OFF_KIND(weapon,Hash40::new("sys_hit_ice"),false,false);
    EffectModule::detach_all(weapon.module_accessor, 5);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("ice"), 0.0, 1.0, false, 0.0, false, false);
    MotionAnimcmdModule::call_script_single(weapon.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_ice"), -1);
    MotionAnimcmdModule::call_script_single(weapon.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_ice"), -1);
    macros::PLAY_SE_REMAIN(weapon, Hash40::new("se_samus_special_n02"));

    let eff = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_ice"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.2, true, 0, 0, 0, 0, 0, false, false);
    WorkModule::set_int(weapon.module_accessor, eff as i32, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
    
}

unsafe extern "C" fn cshot_shoot_main_shift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let motion = MotionModule::motion_kind(weapon.module_accessor);
    if motion != Hash40::new("ice").hash {
        println!("Force ice");
        cshot_shoot_ice(weapon);
    }
    let eff = WorkModule::get_int(weapon.module_accessor,*WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW) as u32;
    EffectModule::set_pos(weapon.module_accessor, eff, &Vector3f::zero());
    EffectModule::set_rot(weapon.module_accessor, eff, &Vector3f::zero());

    let despawn = WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE,0) & 1 != 0;
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) 
    || despawn {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    //set effect following

    0.into()
}
#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn cshot_shoot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let eff = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    println!("Eff End: {eff}");
    let is_ice = charge < 0.0 || eff <= 0;
    if !is_ice {
        return original!(weapon);
    }
    EFFECT_OFF_KIND(weapon,Hash40::new("sys_ice"),false,false);

    let pos = PostureModule::pos(weapon.module_accessor);
    EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_freezer"),
        pos,
        &Vector3f::zero(),
        0.35,
        0,
        -1,
        false,
        0
    );

    return 0.into()
}

#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn cshot_charge_init(weapon: &mut L2CWeaponCommon) -> L2CValue {

    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let eff = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    println!("Charge Init charge: {charge} Eff: {eff} ");

    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner = get_battle_object_from_id(owner_id);
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    if utility::get_kind(&mut *owner_boma) != *FIGHTER_KIND_SAMUS {
        println!("Not samus");
        return 0.into()
    }

    let is_ice = charge < 0.0 || VarModule::is_flag(owner, samus::instance::flag::ICE_MODE);
    if !is_ice {
        return original!(weapon);
    };
    WorkModule::set_float(weapon.module_accessor, -1.0,*WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let min_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_scale"));
    let max_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_scale"));

    let scale = min_scale + (max_scale-min_scale)*charge.abs();
    println!("Charge Scale: {scale}");
    effect!(
        weapon,
        MA_MSC_EFFECT_REQUEST_FOLLOW,
        Hash40::new("sys_ice"),
        Hash40::new("top"),
        7.98004,
        -0.50584,
        -0.25092,
        -91.2728,
        -1.7974,
        176.373,
        scale,
        false,
        0,
        0,
        0
    );
    weapon.clear_lua_stack();
    lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
    sv_module_access::effect(weapon.lua_state_agent);
    let handle = weapon.pop_lua_stack(1).get_i32();
    WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);

    WorkModule::set_int(weapon.module_accessor, -1, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    WorkModule::set_int(weapon.module_accessor, -1, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW_SUB);

    return 0.into()
}


#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn cshot_charge_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner = get_battle_object_from_id(owner_id);
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    if utility::get_kind(&mut *owner_boma) != *FIGHTER_KIND_SAMUS {
        println!("Isabelle");
        let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        let eff = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
        println!("Charge MAIN charge: {charge} Eff: {eff} ");
        weapon.fastshift(L2CValue::Ptr(cshot_charge_main_shift as *const () as _));
        return 0.into();
    }
    
    return original!(weapon);
}
unsafe extern "C" fn cshot_charge_main_shift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn cshot_charge_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let eff = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    println!("Charge End charge: {charge} Eff: {eff} ");
    
    return 0.into()
}
pub fn install() {
    install_status_scripts!(
        //cshot_shoot_main,
        cshot_shoot_init,
        cshot_shoot_end,
        cshot_charge_init,

        cshot_charge_main,
        cshot_charge_end,
    );
}