use leptos::*;

mod navbar;
mod blog;
mod todo;
mod calc;
mod about;

use navbar::NavBar;
use blog::Blog;
use todo::Todo;
use calc::Calc;
use about::About;



#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;

    //let contents = [Blog, Todo, Calc, About];
    let (index, set_index) = create_signal(0);

    view! {

        <NavBar/>
        <Pages/>


        // <button
        //     on:click=move |_| {
        //         set_count.update(|n| *n += 1);
        //     }
        // >
        //     "Click me"
        // </button>
        <br/>
    }
}


#[component]
fn Pages() -> impl IntoView {


    view! {
        <hr/>
        <Blog/>
        <hr/>
        <Todo/>
        <hr/>
        <Calc/>
        <hr/>
        <About/>
        <hr/>
    }

}


fn main() {
    leptos::mount_to_body(HomePage)
}

#[component]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
    let (selected, set_selected) = create_signal(0);
    provide_context(selected);

    let buttons = labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| view! {
            <button on:click=move |_| set_selected(index)>
                {label}
            </button>
        })
        .collect_view();
    view! {
        <div style="display: flex; width: 100%; justify-content: space-around;">
            {buttons}
        </div>
        {children()}
    }
}


// #[component]
// fn Tab(index: usize, children: Children) -> impl IntoView {
//     view! {
//         <div>{children()}</div>
//     }
// }
#[component]
fn Tab(index: usize, children: Children) -> impl IntoView {
    let selected = expect_context::<ReadSignal<usize>>();
    view! {
        <div style:display=move || if selected()==index {
            "block"
        } else {
            "none"
        }>
            {children()}
        </div>
    }
}

#[component]
fn Tabs2(labels: Vec<String>, children: Children) -> impl IntoView {
    let buttons = labels
        .into_iter()
        .map(|label| view! { <button>{label}</button> })
        .collect_view();
    view! {
        <div style="display: flex; width: 100%; justify-content: space-around;">
            {buttons}
        </div>
        {children()}
    }
}


#[component]
fn HomePage() -> impl IntoView {
    let files = ["a.txt", "b.txt", "c.txt"];
    let labels = files.iter().copied().map(Into::into).collect();
    let tabs = move || {
        files
            .into_iter()
            .enumerate()
            .map(|(index, filename)| {
                let content = filename;
                view! {
                    <Tab index>
                        <h2>{filename.to_string()}</h2>
                        <p>{content}</p>
                    </Tab>
                }
            })
            .collect_view()
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <p>"Click any of the tabs below to read a recipe."</p>
        <Tabs labels>
            <div>{tabs()}</div>
        </Tabs>
    }
}

