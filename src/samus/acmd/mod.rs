mod specials;
mod crawl;
mod dtilt;
mod fsmash;
mod upsmash;
mod sidetaunt;

pub fn install() {
    specials::install();
    crawl::install();
    dtilt::install();
    fsmash::install();
    upsmash::install();
    sidetaunt::install();
}