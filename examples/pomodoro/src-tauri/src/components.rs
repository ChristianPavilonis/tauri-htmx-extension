#![allow(non_snake_case)]

use std::time::Duration;

use shtml::{html, Component, Elements, Render};

pub fn Layout() -> Component {
    html! {
        <div class="bg-gray-800 text-white min-h-[100vh] flex justify-center items-center">
           <Main /> 
        </div>
    }
}

pub fn Main() -> Component {
    html! {
        <div>
            <form tauri-invoke="set_task_description" class="flex flex-col space-y-4 mb-8">
                <label for="description"> Whatcha working on? </label>
                <input type="text" name="description" class="bg-gray-400 text-black" />
                <button type="submit" class="bg-gray-700">Set</button>
            </form>
            <div>
                <button tauri-invoke="pomodoro" hx-swap="outerHTML" class="bg-gray-700 p-2">
                    Start working
                </button>
                <div tauri-listen="tick"></div>
            </div>
        </div>
    }
}

fn format_seconds(mut seconds: u32) -> String {
    let min = seconds / 60;
    seconds = seconds - min * 60;

    format!("{:0>2}:{:0>2}", min, seconds)
}

pub fn Timer(seconds: u32, pomodoros: u16) -> Component {
    html! {
        <div>
            <p class="text-3xl">{format_seconds(seconds)}</p>
            <p class="text-lg">{format!("{pomodoros} completed pomodoros")}</p>
        </div>
    }
}

pub fn TaskDescription(description: &str) -> Component {
    html! {
        <p class="text-xl">
            {description}
        </p>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_formats_seconds() {
        assert_eq!(format_seconds(830), "13:50".to_string());
    }
}
