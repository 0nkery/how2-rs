mod google;

use google::Google;

fn main() {
    let google = Google::default();
    let links = google.google("rust");

    println!("{:?}", links);
}
