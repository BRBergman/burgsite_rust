use yew::prelude::*;
use yew_router::prelude::*;
pub mod home;
use home::*;
pub mod switcher;
pub use switcher::*;
//https://yew.rs/docs/getting-started/build-a-sample-app



#[function_component(BlahajGallery)]
fn blahaj_gallery() -> Html {

    html! {
    

<html>

<head>
  <title>{"The Blahaj Gallery"}</title>
  //<link rel="icon" href="https://burgburg.net/Blah/blahajIcon.png">
  <style />
</head>

<body style="background: #f5acba; text-align: center;">
  <div style="padding: 30px; text-align: left">
    <a style="color: #60d0fa; text-shadow: 3px 3px 5px black" href="/">{"return..."}</a>
  </div>
  <img src="https://burgburg.net/BlahajGallery/Blah1.JPG" />
  <img src="https://burgburg.net/BlahajGallery/Blah4.JPG" />
  <img src="https://burgburg.net/BlahajGallery/Blah5.jpg" />
  <img src="https://burgburg.net/BlahajGallery/Blah2.JPG" style="width: 45%" />
  <img src="https://burgburg.net/BlahajGallery/Blah3.JPG" style="width: 45%" />
</body> 
</html>
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
         <Home />
        },
        Route::BlahajGallery => html! {
            <BlahajGallery />
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