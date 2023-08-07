pub mod samus {
    pub const MORPH_FRAME: f32 = 7.0; //12.0
    pub const LOOP_FRAME: f32 = 27.0; //32.0
    pub const UNMORPH_FRAME: f32 = 30.0;
    pub const BOMB_COOLDOWN_MAX: i32 = 45;
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_LW_CAN_EXIT: i32 = 0x0100;
            pub const SPECIAL_LW_EXIT: i32 = 0x0101;
            pub const ICE_MODE: i32 = 0x0102;
        }
        pub mod int {
            pub const SPECIAL_LW_ROTATIONS: i32 = 0x0100;
            pub const BOMB_COOLDOWN: i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const ATTACK_LW3_ICE_PILLAR: i32 = 0x1000;
        }
    }
}