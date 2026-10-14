mod largest;
mod point;
mod point_f32;
mod point_mixup;

fn main() {
    largest::get_largest_int_and_char();
    point::display_points();
    point_f32::display_points();
    point_mixup::display_points();
}
