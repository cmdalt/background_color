use leptos::*;
fn main() {
    leptos::mount_to_body(|cx| view! {cx, <App/>})
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {

  let (farbe, set_farbe) = create_signal(cx, false);
  let (colors,set_colors) = create_signal(cx, "black".to_string());

  view! { cx,
  <div
  style:background-color=move || format!("{}",colors())
  style="
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction:column;
  color:white;
  width:100%;
  height: 100%;
  ">
  
  <My_button farbes=set_farbe color=set_colors/>

  <h1>{farbe}</h1>
  
  </div>
  }

}

#[component]
pub fn My_button(cx: Scope,farbes:WriteSignal<bool>,color:WriteSignal<String>) -> impl IntoView {

  view! { cx,
  <div>
    <button on:click= move |_| {
      farbes.update(|x| *x = !*x);
      farbes.update(|x| if *x == true { 
        color.update(|x| *x = "white".to_string())
      } else {
        color.update(|x| *x = "black".to_string())
      }
    )
    }
    >
    "deneme"
    </button>
  </div>
  }

}
