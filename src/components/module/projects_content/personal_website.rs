use crate::components::shared::back_project_button::BackProjectButton;
use yew::{function_component, html, Html};

#[function_component(PersonalWebsite)]
pub fn personal_website() -> Html {
    html! {
        <div class="main projects-frame">
            <div class="content-frame">
                <div class="content">
                    <BackProjectButton />
                    <div>
                        {"
                        Hanifanrn Personal Website with Rust

                        Repository [Link]
                        
                        Figma [Link]
                        
                        I love something ***sophisticated***, something that is mindfully crafted with a well-thought-out design.
                        
                        But when I reflect on the personal projects that I have done in the past few years, I feel that my works are not ***sophisticated*** enough. there is a tendency to just move to a new project before the current project is ready to be released to the public or else I just ship it to the public without really thinking about the quality.
                        
                        Those two sentences contradict each other and the moment I realized it, it got me thinking:
                        
                        “I want to create a personal website with a well-thought-out design. A personal website that can contain my other project. Something that I can feel proud of.”
                        
                        - Previously I had a personal website using an HTML5 UP design template, I found it very cool, but I want to create something from scratch [link]
                        
                        The motivation is set up, and the next thing is doing some research on how the website should be designed.
                        
                        After exploring the Internet, I found that there is some value I want to deliver:
                        
                        1. I want the website not to be a one-page scroll-oriented website.
                        
                        2. I want the navigation and contact list to be always present.
                        
                        3. Most importantly I want to write it in Rust.
                        
                        With that in mind, I started designing my website in Figma [link]
                        
                        If you look at my Figma Design, I use a lot of Golden Ratio and fibonacci number as my padding, margin, font size and even the RGB color number.
                        
                        It is because I want my design not ***centrally*** oriented, and then I remembered that I could use Fibonacci numbers for my website's positioning, padding, and margin so that even if the positioning is not ***centrally*** oriented, the overall content placement is not totally random so the user can still somehow feel that my website still has a structure.
                        
                        Here are some of the websites that influenced my website design:
                        
                        1. https://dpbriggs.ca/
                        
                        I first found this website because I wanted to write my website using Rust, and then I found out that I love BOX! It makes me more focused on the specific part of the website.
                        
                        2. https://p5aholic.me/
                        
                        I love how the website content is positioned! The difference is that I want my contact list to be always present.
                        
                        3. https://v4.brittanychiang.com/
                        
                        Where I get the idea that I want my contact always be present in the web page.
                        
                        Those 3 website influence my current website design as you can see it right now.
                        
                        As for the tech stack, previously in 2023 I only use html, css and js for static website that get deployed on github[dot]io because I still don’t know how to use Rust and after around a year learning about it I decided to rewrite it in rust using Yew framework.
                        
                        Previously I rewrite it to leptos but since I will serve it as a static website, I need hashroute and by the time I finnish rewrite the core functionality that I realise that Leptos still not support hashroute :) so I rewrite it again using Yew.
                        "}
                    </div>
                </div>
            </div>
        </div>
    }
}
