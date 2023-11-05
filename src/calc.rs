use leptos::*;


#[component]
pub fn Calc() -> impl IntoView {

    view! {

        <div class="container">
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