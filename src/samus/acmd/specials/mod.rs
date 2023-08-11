mod speciallw_ball;
mod speciallw;
mod specialn;
mod missile;

pub fn install() {
    specialn::install();
    speciallw::install();
    missile::install();
    //speciallw_ball::install();
}