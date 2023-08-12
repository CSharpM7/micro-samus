mod speciallw;
mod crawl;
//mod crawl_common;
mod crawl_shared;
mod ice_attacks;
mod specialn;
mod missiles;

pub fn install() {
    speciallw::install();
    //specialn::install();
    crawl::install();
    missiles::install();
    ice_attacks::install();
}

