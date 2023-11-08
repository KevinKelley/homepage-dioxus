use leptos::*;
use leptos_router::*;

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



// #[component]
// fn Tab(index: usize, children: Children) -> impl IntoView {
//     let selected = expect_context::<ReadSignal<usize>>();
    
//     view! {
//         <div style:display=move || if true { //if selected()==index {
//             "block"
//         } else {
//             "none"
//         }>
//             {children()}
//         </div>
//     }
// }
// #[component]
// fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
//     let (selected, set_selected) = create_signal(0);
//     provide_context(selected);

//     let buttons = labels
//         .into_iter()
//         .enumerate()
//         .map(|(index, label)| view! {
//             <button on:click=move |_| set_selected(index)>
//                 {label}
//             </button>
//         })
//         .collect_view();
//     view! {
//         <div style="display: flex; width: 100%; justify-content: space-around;">
//             {buttons}
//         </div>
//         // {children()}
//         <Tab index=0><Blog/></Tab>
//         <Tab index=1><Todo/></Tab>
//         <Tab index=2><Calc/></Tab>
//         <Tab index=3><About/></Tab>
//     }
// }


// #[component]
// fn Home() -> impl IntoView {

//     // let (toggled, set_toggled) = create_signal(false);
//     // // share `set_toggled` with all children of this component
//     // provide_context(set_toggled);

//     let names = ["Blog", "Todo", "Calc", "About"];
//     let labels = names.iter().copied().map(Into::into).collect();
//     let tabs = move || {
//         names
//             .into_iter()
//             .enumerate()
//             .map(|(index, filename)| {
//                 let content = filename;
//                 view! {
//                     <Tab index>
//                         <h2>{filename.to_string()}</h2>
//                         <p>{content}</p>
//                     </Tab>
//                 }
//             })
//             .collect_view()
//     };

//     view! {
//         <Tabs labels>
//             <div>{tabs()}</div>
//         </Tabs>
//     }
// }


#[component]
fn TitleBar() -> impl IntoView {
    view! {
        <h1> kevkevDEV </h1>   
        <NavBar/>    
    }
}

///////////////// GLOBAL STATE (well, local to the tree where we install it)
#[derive(Copy, Clone, Debug)]
struct GlobalState {
    count: RwSignal<i32>,
    name: RwSignal<String>
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            count: create_rw_signal(0),
            name: create_rw_signal("Bob".to_string())
        }
    }
}



#[component]
pub fn App() -> impl IntoView {
    // provide_context(GlobalState::new());

    view! {
        <Router>
            <main style="background-color: lightblue; padding: 10px">
                <Routes>
                    <Route path="" view=|| view! { <h1>"Hello World!"</h1> }/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }

//   view! {    
    // <TitleBar/>
    // <Router>
    //   <nav>
    //   //<TitleBar/>
    //   <h1>kevkevDEV site<h1>
    //   </nav>
    //   <main>
    //     // all our routes will appear inside <main>
    //     <Routes>
    //     <Route path="/" view=About/>
    //     <Route path="/blog" view=Blog/>
    //     <Route path="/todo" view=Todo/>
    //     <Route path="/calc" view=Calc/>
    //     <Route path="/about" view=About/>
    //     // <Route path="/users" view=Users/>
    //     // <Route path="/users/:id" view=UserProfile/>
    //     <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
    //     </Routes>
    //   </main>
    // </Router>
//   }
}

