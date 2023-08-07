mod speciallw;
mod crawl;
//mod crawl_common;
mod crawl_shared;
mod other;

pub fn install() {
    speciallw::install();
    crawl::install();
    other::install();
}

