use leptos::*;
// use styled::style;
// use styled::Scope;


#[component]
pub fn Calc() -> impl IntoView {

    // let styles = style!(
    //     body {
    //         width: 100%;
    //         background-color: #dbdbdb;
    //     }
    //     .container {
    //         display: flex;
    //         width: inherit;
    //         flex-direction: column;
    //         justify-content: center;
    //         margin-top: 15px;
    //         align-items: center;
    //     }
    //     .wrapper {
    //         width: 500px;
    //     }
    //     .col {
    //         width: 500px;
    //         display: flex;
    //         flex-direction: row;
    //         justify-content: center;
    //         align-items: center;
    //     }
    //     .expression {
    //         background-color: white;
    //         width: inherit;
    //         height: 100px;
    //         font-size: 50px;
    //         border-top: 1px solid gray;
    //         border-right: 1px solid gray;
    //         border-left: 1px solid gray;
    //     }
    //     .result {
    //         width: inherit;
    //         margin-bottom: 12px;
    //         height: 54px;
    //         background-color: white;
    //         border-bottom: 1px solid gray;
    //         border-right: 1px solid gray;
    //         border-left: 1px solid gray;
    //     }
    //     button {
    //         margin-right: 2px;
    //         margin-top: 2px;
    //         width: inherit;
    //         font-weight: 700;
    //         border: 2px solid gray;
    //         margin-bottom: 2px;
    //         font-size: 43px;
    //         text-align: center;
    //     }
    //     @media (max-width: 500px) {
    //         .wrapper {width: 100%;}
    //         .container {width: 100%;}
    //         .col {width: 100%;}
    //         body {width: inherit;}
    //     }
            
    //   );
    
    //styled::
    view! {
    
        <div class="container" id="calculator">
        <div class="wrapper">
            <div class="col">
            <div class="expression" id="expression">Expression</div> 
            </div>
            <div class="col">
            <div class="result" id="result">Result</div>
            </div>
            <div class="col">
            <button>1</button>
            <button>2</button>
            <button>3</button>
            <button>-</button>
            </div>
            <div class="col">
            <button>4</button>
            <button>5</button>
            <button>6</button>
            <button>+</button>
            </div>
            <div class="col">
            <button>7</button>
            <button>8</button>
            <button>9</button>
            <button>*</button>
            </div>
            <div class="col">
            <button>CLR</button>
            <button>0</button>
            <button>=</button>
            <button>/</button>
            </div>
        </div>
        </div>
    }
   
}