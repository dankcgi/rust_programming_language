mod creating;
mod accessing;
mod updating;

fn main() {
    let scores = creating::create_hash_map();
    let blue_score = accessing::get_team_score(String::from("Blue"), scores);
    println!("Blue score is {blue_score}");

    let new_scores = creating::create_hash_map();
    accessing::iterate_scores(new_scores);

    updating::overwrite();
    updating::add_if_not_present();
    updating::update_based_on_old_value();
}
