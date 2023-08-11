mod speciallw;
mod crawl;
//mod crawl_common;
mod crawl_shared;
mod other;
mod ice_attacks;
mod specialn;
mod cshot;

pub fn install() {
    speciallw::install();
    specialn::install();
    cshot::install();
    crawl::install();
    other::install();
    ice_attacks::install();
}

