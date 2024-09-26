use yew::prelude::*;
use yew_router::prelude::*;
pub mod home;
use home::*;
pub mod switcher;
pub use switcher::*;
//https://yew.rs/docs/getting-started/build-a-sample-app


#[function_component(BlahajGalleryPage)]
fn blahaj_gallery() -> Html {
    html! {
    <div style="background: #f5acba; text-align: center; margin: 0px">
      <div style="padding: 30px; text-align: left; background: #bbd8bb;">
       <div ><TabSwitcher/></div>
      </div>
      <img src="https://burgburg.net/BlahajGallery/Blah1.JPG" style="width: 30%;"/>
      <img src="https://burgburg.net/BlahajGallery/Blah4.JPG" style="width: 30%;"/>
      <img src="https://burgburg.net/BlahajGallery/Blah5.jpg" style="width: 30%;"/>
      <img src="https://burgburg.net/BlahajGallery/Blah2.JPG" style="width: 45%" />
      <img src="https://burgburg.net/BlahajGallery/Blah3.JPG" style="width: 45%" />
    </div>
    }
}

#[function_component()]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
         <HomePage />
        },
        Route::BlahajGallery => html! {
            <BlahajGalleryPage />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::BurgBlog => html! {
        <div>{"hi"}</div>
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
