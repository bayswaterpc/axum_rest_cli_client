mod reqwest_readme;
pub mod spotify_example;
pub mod todos;

use futures::join;
use spotify_example::spotify_request;

pub async fn make_requests() {
    let fut1 = spotify_request();
    let fut2 = reqwest_readme::reade_me_printer();

    join!(fut1, fut2);
}
