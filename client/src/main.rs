use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(ShowAllPatients)]
fn show_all_patients() -> Html {
    html! {
        <div>
            <h1>{"Hello, World!"}</h1>
        </div>
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[at("/")]
    ShowAllPatients,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::ShowAllPatients => html! {<ShowAllPatients/>},
        Route::NotFound => html! { <h1>{"404"}</h1>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
