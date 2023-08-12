mod speciallw_ball;
mod speciallw;
mod specialn;
mod missiles;

pub fn install() {
    specialn::install();
    speciallw::install();
    missiles::install();
    //speciallw_ball::install();
}