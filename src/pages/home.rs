use crate::components::{about::About, hero::Hero, projects::Projects, contact::Contact};
use crate::components::{footer::Footer, nav::Nav};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">
                <Nav/>
                <Hero/>
                <Projects/>
                <About/>
                <Contact/>
                <Footer/>
            </div>

        </ErrorBoundary>
    }
}
