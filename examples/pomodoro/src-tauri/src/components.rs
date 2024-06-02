#![allow(non_snake_case)]

use std::time::Duration;

use shtml::{html, Component, Elements, Render};

pub fn Layout() -> Component {
    html! {
        <div>
           <Main /> 
        </div>
    }
}

pub fn Main() -> Component {
    html! {
        <form tauri-invoke="set_task_description">
            <input type="text" name="description" />
            <button type="submit">Set</button>
        </form>
        <div>
            <button tauri-invoke="pomodoro">Start</button>
            <div tauri-listen="tick"></div>
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
            <p>{format_seconds(seconds)}</p>
            <p>{format!("{pomodoros} completed pomodoros")}</p>
        </div>
    }
}

pub fn TaskDescription(description: &str) -> Component {
    html! {
        <p>
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
