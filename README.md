# Tauri + htmx

## Examples

To use the examples clone the repo, cd into the example, you'll have to create a hard link to the htmx.js and tauri-ext.js files.
```sh
cd examples/hello
ln ../htmx.js src/
ln ../../src/tauri-ext.js src/

cargo tauri dev
```

### Goals

The goal is to make it easy to template and write views on the rust side of the tauri app with no need for wasm and using htmx for the dynamic bits.

## Usage

### Setup


To get started copy htmx and the tauri-ext.js files into your src folder, then load htmx and the extension, then register the extension using `hx-ext`

```html
<head>
    ...
    <script type="module" src="/tauri-ext.js" defer></script>
    <script src="/htmx.js" defer></script>
</head>
<body hx-ext="tauri"></body>
```

### Invoke

To invoke a `tauri::command` use the `tauri-invoke` attribute

```html
<button tauri-invoke="hello" hx-target="#my-div">Click me</button>
<div id="my-div"></div>
```

```rust
#[tauri::command]
fn hello() -> String {
    "Hello, world!".to_string()
}
```

This is the same as calling `invoke('hello')` then it will insert "Hello, world!" into #my-div.

It also works with forms.

```html
<form tauri-invoke="save" hx-swap="outerHTML">
    <input type="text" name="name" />
    <button type="submit">Save</button>
</form>
```

```rust
#[tauri::command]
async fn save(name: &str) -> Result<String, String> {
    do_save(name).await;

    format!("Saved {name} to contacts")
}
```

This is the same as calling `invoke('save', {name})`, hx-swap will swap the form for our success message.

### Listen

You can also listen to events. For example if you have a background task working and wish to update the frontend with a progress.

```html
<div tauri-listen="progress"></div>
```

```rust
fn do_stuff(app: AppHandle) {
    thread::spawn(move || {
        for i in 0..=100 {
            app.emit("progress", format!("progress: {i}%")).unwrap();

            thread::sleep(Duration::from_millis(100));
        }
    });
}
```

This uses the `event.listen()` tauri function and will use htmx to swap the innerHTML of the div. You can still use hx-target.

## Notes

When it comes to rust templating [shtml](https://github.com/swlkr/shtml) is pretty nice, but there are other options.

Also checkout the [SHAT STACK](https://github.com/ChristianPavilonis/shat-stack) a full stack rust/htmx app template using shtml, htmx, axum, and tailwind. (also a wip)
