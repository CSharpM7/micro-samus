mod speciallw;
mod crawl;
//mod crawl_common;
mod crawl_shared;
mod other;
mod ice_attacks;

pub fn install() {
    speciallw::install();
    crawl::install();
    other::install();
    ice_attacks::install();
}

