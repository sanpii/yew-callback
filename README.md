# yew-callback

[![Github actions](https://github.com/sanpii/yew-callback/actions/workflows/ci.yml/badge.svg)](https://github.com/sanpii/yew-callback/actions/workflows/ci.yml)
[![Gitlab CI](https://gitlab.com/sanpi/yew-callback/badges/main/pipeline.svg)](https://gitlab.com/sanpi/yew-callback/commits/main)

Easily create yew callback.

```rust,ignore
yew_callback::callback!(state, name = props.name, move |_| {});
```

Will be expanded as:
```rust,ignore
{
    let state = state.clone();
    let name = props.name.clone();

    yew::Callback::from(move |_| {});
}
```
