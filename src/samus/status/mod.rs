mod speciallw;
mod crawl;
//mod crawl_common;
mod crawl_shared;
mod ice_attacks;
mod specialn;
mod specials;
mod missiles;

pub fn install() {
    speciallw::install();
    specials::install();
    specialn::install();
    crawl::install();
    missiles::install();
    ice_attacks::install();
}

