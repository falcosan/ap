use dioxus::prelude::*;
use reqwest::Error;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub(crate) fn Dogs() -> Element {
    let mut img_src = use_signal(|| "".to_string());

    async fn fetch_dog_image() -> Result<String, Error> {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random").await?;
        let data: DogApi = response.json().await?;
        Ok(data.message)
    }

    let fetch_new = move |_| async move {
        if let Ok(new_img_src) = fetch_dog_image().await {
            img_src.set(new_img_src);
        } else {
            eprintln!("Failed to fetch new image");
        }
    };

    use_future(move || async move {
        if let Ok(initial_img_src) = fetch_dog_image().await {
            img_src.set(initial_img_src);
        } else {
            eprintln!("Failed to fetch initial image");
        }
    });

    rsx! {
        div { class: "w-full h-full flex flex-col items-center justify-center gap-10 py-10 bg-neutral-200",
            div { class: "size-190 flex flex-col items-center justify-center",
                img { class: "rounded shadow", src: "{img_src}" }
            }
            div {
                button {
                    class: "px-4 py-3 rounded cursor-pointer bg-neutral-300",
                    onclick: fetch_new,
                    "generate"
                }
            }
        }
    }
}
