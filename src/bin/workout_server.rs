use bytes::Bytes;
use clap::Parser;
use futures::stream::{self, Stream, StreamExt};
use std::error::Error;
use std::path::PathBuf;
use warp::Filter;
use workout::App;

type BoxedError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    //#[structopt(short, long, parse(from_os_str))]
    dir: PathBuf,
}

async fn render(
    index_html_before: String,
    index_html_after: String,
) -> Box<dyn Stream<Item = Result<Bytes, BoxedError>> + Send> {
    let renderer = yew::ServerRenderer::<App>::new();

    Box::new(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(|m| Result::<_, BoxedError>::Ok(m.into())),
    )
}

#[tokio::main]
async fn main() {
    let opts: Opt = Opt::parse();

    let index_html_s: String = tokio::fs::read_to_string(opts.dir.join("../www/videos.html"))
        .await
        .expect("failed to read www/videos.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();

    let html = warp::path("videos.html").then(move || {
        let index_html_before = index_html_before.clone();
        let index_html_after = index_html_after.clone();

        async move { warp::reply::html(render(index_html_before, index_html_after).await) }
    });

    let root = warp::path::end().and(warp::fs::file("index.html"));
    let routes = root.or(html).or(warp::fs::dir("www"));

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
