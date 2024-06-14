use crate::projects_data::PROJECTS;
use leptos::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section id="projects">
        <h2 style="text-align:center">My Projects</h2>
        <div class="slideshow-container">
            <div class="mySlides" style="display:block">
                    <h3>Treemap Diagram</h3>
                    <iframe
                        height="700"
                        style="width: 100%;"
                        scrolling="no"
                        title="Treemap Diagram"
                        src="https://codepen.io/demi0040/embed/yLqJjEM?default-tab=result"
                        frameborder="no"
                        loading="lazy"
                        allowtransparency="true"
                        allowfullscreen="true">
                    </iframe>
                </div>
            {PROJECTS.iter().enumerate().map(|(_index, project)| view! {
                <div class="mySlides fade">
                    <h3>{project.title}</h3>
                    <iframe
                        height="700"
                        style="width: 100%;"
                        scrolling="no"
                        title={project.title}
                        src={project.src}
                        frameborder="no"
                        loading="lazy"
                        allowtransparency="true"
                        allowfullscreen="true">
                    </iframe>
                </div>
            }).collect_view()}
            <a class="prev" onclick="plusSlides(-1)">q</a>
            <a class="next" onclick="plusSlides(1)">p</a>
        </div>
        <br/>
        <div style="text-align:center">
            {PROJECTS.iter().enumerate().map(|(index, _)| view! {
                <span class="dot" onclick={format!("currentSlide({})", index + 1)}></span>
            }).collect_view()}
        </div>
    </section>
    }
}
