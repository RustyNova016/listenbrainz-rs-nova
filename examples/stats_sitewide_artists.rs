use listenbrainz_rs_nova::raw::Client;

fn main() {
    let client = Client::new();

    let result = client.stats_sitewide_artists(None, None, Some("year"));
    println!("{:#?}", result);
}
