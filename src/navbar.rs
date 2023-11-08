use leptos::*;



// </style>
//   <header>
//     <nav class="nav-wrapper">
//       <ul class="main-nav">
//         <li class="menu-item"><a href="#">Home</a></li>
//         <li class="menu-item"><a href="#">About</a></li>
//         <li class="menu-item"><a href="#">Contact</a></li>
//       </ul>
//       <ul class="right-side-nav">
//         <li class="menu-item" id="login"><a href="#">Login</a></li>
//         <li class="menu-item" id="register"><a href="#">Register</a></li>
//       </ul>
//     </nav>
//   </header>


#[component]
pub fn NavBar() -> impl IntoView {

let labels = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string(), "Delta".to_string(), "Rho".to_string()];

view! {

    //<Tabs labels=labels/>

// <style>
// body{
// margin: 0;
// padding: 0;
// }
// .nav-wrapper {
// display: flex;
// flex-wrap: wrap;
// justify-content: space-between;
// background-color: #333;
// height: 50px;
// width: 100%;
// }
// .main-nav, .right-side-nav {
// display: flex;
// align-items: center;
// margin: 0;
// padding: 0;
// list-style-type: none;
// }
// .menu-item {
// display: flex;
// flex-direction: column;
// justify-content: center;
// flex-wrap: wrap;
// padding: 0 10px 0 10px;
// height: 100%;
// }
// .menu-item a {
//   text-decoration: none;
//   color: #fff;
// }
// .menu-item hover {
//   background-color: #111;
//   cursor: pointer;
// }


    <header>
      <nav class="nav-wrapper">
        <ul class="main-nav">
          <li class="menu-item"><a href="#">Home</a></li>
          <li class="menu-item"><a href="#">Blog</a></li>
          <li class="menu-item"><a href="#">ToDo</a></li>
          <li class="menu-item"><a href="#">Calc</a></li>
          <li class="menu-item"><a href="#">About</a></li>
        </ul>
        <ul class="right-side-nav">
          <li class="menu-item" id="login"><a href="#">Login</a></li>
          <li class="menu-item" id="register"><a href="#">Register</a></li>
        </ul>
      </nav>
    </header>

}
}


// /////////////////////////////////////////////////////////////////
// #[component]
// fn HomePage() -> impl IntoView {
//     // these are the files we’re going to read
//     let files = ["a.txt", "b.txt", "c.txt"];
//     // the tab labels will just be the file names
//     let labels = files.iter().copied().map(Into::into).collect();
//     view! {
//         <h1>"Welcome to Leptos!"</h1>
//         <p>"Click any of the tabs below to read a recipe."</p>
//         <Tabs labels/>
//     }
// }

// // from the "islands" example
// #[component]
// fn Tabs(labels: Vec<String>) -> impl IntoView {
//     let buttons = labels
//         .into_iter()
//         .map(|label| view! { <button>{label}</button> })
//         .collect_view();
//     view! {
//         <div style="display: flex; width: 100%; justify-content: space-between;">
//             {buttons}
//         </div>
//     }
// }
// /////////////////////////////////////////////////////////////
// #[component]
// fn Tab(index: usize, children: Children) -> impl IntoView {
//     view! {
//         <div>{children()}</div>
//     }
// }
// #[component]
// fn Tabs2(labels: Vec<String>, children: Children) -> impl IntoView {
//     let buttons = labels
//         .into_iter()
//         .map(|label| view! { <button>{label}</button> })
//         .collect_view();
//     view! {
//         <div style="display: flex; width: 100%; justify-content: space-around;">
//             {buttons}
//         </div>
//         {children()}
//     }
// }


// #[component]
// fn HomePage2() -> impl IntoView {
//     let files = ["a.txt", "b.txt", "c.txt"];
//     let labels = files.iter().copied().map(Into::into).collect();
//     let tabs = move || {
//         files
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
//         <h1>"Welcome to Leptos!"</h1>
//         <p>"Click any of the tabs below to read a recipe."</p>
//         <Tabs2 labels>
//             <div>{tabs()}</div>
//         </Tabs2>
//     }
// }













// /*
// <html>
// <head>
//   <title>(Flexbox)</title>
//   <meta name="viewport" content="width=device-width,initial-scale=1">
// </head>
// <body>
// <style>
// body{
// margin: 0;
// padding: 0;
// }
// .nav-wrapper {
// display: flex;
// flex-wrap: wrap;
// justify-content: space-between;
// background-color: #333;
// height: 50px;
// width: 100%;
// }
// .main-nav, .right-side-nav {
// display: flex;
// align-items: center;
// margin: 0;
// padding: 0;
// list-style-type: none;
// }
// .menu-item {
// display: flex;
// flex-direction: column;
// justify-content: center;
// flex-wrap: wrap;
// padding: 0 10px 0 10px;
// height: 100%;
// }
// .menu-item a {
//   text-decoration: none;
//   color: #fff;
// }
// .menu-item hover {
//   background-color: #111;
//   cursor: pointer;
// }
// </style>
//   <header>
//     <nav class="nav-wrapper">
//       <ul class="main-nav">
//         <li class="menu-item"><a href="#">Home</a></li>
//         <li class="menu-item"><a href="#">Blog</a></li>
//         <li class="menu-item"><a href="#">Todo</a></li>
//         <li class="menu-item"><a href="#">HP11c</a></li>
//         <li class="menu-item"><a href="#">About</a></li>
//         <li class="menu-item"><a href="#">Contact</a></li>
//       </ul>
//       <ul class="right-side-nav">
//         <li class="menu-item" id="login"><a href="#">Login</a></li>
//         <li class="menu-item" id="register"><a href="#">Register</a></li>
//       </ul>
//     </nav>
//   </header>

// </body>
// </html>
// */