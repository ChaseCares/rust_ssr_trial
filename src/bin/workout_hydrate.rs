fn main() {
    #[cfg(target_arch = "wasm32")]
    yew::Renderer::<App>::new().hydrate();
}
