#[test]
fn no_param() {
    let _ = yew_callback::callback!(move |_: ()| {});
}

#[test]
fn param() {
    let _param = "";

    let _ = yew_callback::callback!(_param, move |_: ()| {});
}

#[test]
fn params() {
    let _param1 = "";
    let _param2 = "";

    let _ = yew_callback::callback!(_param1, _param2, move |_: ()| {});
}

#[test]
fn alias() {
    let _param1 = "";

    let _ = yew_callback::callback!(_param1, _param2 = "", move |_: ()| {});
}
