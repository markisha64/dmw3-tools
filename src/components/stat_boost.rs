use dioxus::prelude::*;

type Stack = (&'static str, i64);

#[component]
pub fn StatBoost(
    label: Option<&'static str>,
    id: &'static str,
    cb: EventHandler<i64>,
    options: &'static [Stack],
) -> Element {
    let mut value = use_signal::<i64>(|| 0);
    let mut stacks = use_signal::<Vec<Stack>>(|| vec![]);

    let c_value = value();

    let lb = label.unwrap_or("Stat boost");

    rsx! {
        "{lb}",
        button {
            onclick: move |_| {
                let _e = eval(format!("document.getElementById(\"{id}\").showModal();").as_str());
            },
            "New stack"
        }
        dialog {
            id,
            span {
                onclick: move |_| {
                    let _e = eval(format!("document.getElementById(\"{id}\").close();").as_str());
                },
                class: "close",
                "Close"
            }
            for option in options {
                button {
                    onclick: move |_| {
                        let _e = eval(format!("document.getElementById(\"{id}\").close();").as_str());

                        let new_value = (c_value + option.1).clamp(-64, 128);

                        value.set(new_value);

                        stacks.write().push(option.clone());

                        cb(new_value);
                    },
                    "{option.0}"
                }
            }
        }
        ul {
            class: "stacks",
            for (idx, stack) in stacks.read().iter().enumerate() {
                li {
                    "{stack.0}",
                    span {
                        onclick: move |_| {
                            stacks.write().remove(idx);

                            let mut new_value = 0;

                            for s in stacks.read().iter() {
                                new_value = (new_value + s.1).clamp(-64, 128);
                            }

                            value.set(new_value);

                            cb(new_value);
                        },
                        class: "close",
                        "Remove"
                    }
                }
            }
        }
    }
}
