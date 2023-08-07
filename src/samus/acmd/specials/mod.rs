mod speciallw_ball;
mod speciallw;
mod missile;

pub fn install() {
    speciallw::install();
    missile::install();
    //speciallw_ball::install();
}