mod specials;
mod crawl;
mod dtilt;

pub fn install() {
    specials::install();
    crawl::install();
    dtilt::install();
}