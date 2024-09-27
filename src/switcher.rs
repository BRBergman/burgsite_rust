use yew::prelude::*;
use yew_router::prelude::*;
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/404")]
    NotFound,
    #[at("/BlahajGallery")]
    BlahajGallery,
    #[at("/BurgBlog")]
    BurgBlog,
    
}

#[function_component(TabSwitcher)]
pub fn tab_switcher() -> Html {
    let navi_home = use_navigator().unwrap();
    let navi_bh = navi_home.clone();
    let navi_burgblog = navi_home.clone();
    let home = Callback::from(move |_| navi_home.push(&Route::Home));
    let blahajgallery = Callback::from(move |_| navi_bh.push(&Route::BlahajGallery));
    let burgblog = Callback::from(move |_| navi_burgblog.push(&Route::BurgBlog));
    html! {
        <div>
            <h3>{ "Pages:" }</h3>
            <p> <a onclick={home}>{ "Main Page" }</a></p>
            <p><a onclick={blahajgallery}> {"Blahaj Gallery"}</a></p>
            <p><a onclick={burgblog}> {"The BurgBlog"}</a></p>
        </div>
    }
}
