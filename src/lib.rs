use yew::prelude::*;

fn get_videos() -> Vec<String> {
    let video_dirs = std::fs::read_dir("www/videos").unwrap();

    let mut video_names = Vec::new();
    for video in video_dirs {
        let video_name = video.unwrap().file_name().into_string().unwrap();
        video_names.push(video_name);
    }
    video_names
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <h1>{"Videos"}</h1>
            <ol>
                {for get_videos().iter().map(|video| html! {
                    <li><a href={format!("video.html?video_name={}", video)}>{video.replace('_', " ")}</a></li>
                })}
            </ol>
        </div>
    }
}
