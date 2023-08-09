mod specials;
mod crawl;
mod dtilt;
mod fsmash;
mod fair;
mod upsmash;
mod sidetaunt;
mod ice;

pub fn install() {
    specials::install();
    crawl::install();
    dtilt::install();
    fsmash::install();
    fair::install();
    upsmash::install();
    sidetaunt::install();
    ice::install();
}