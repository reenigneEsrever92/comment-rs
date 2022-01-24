use std::rc::Rc;

use comments_rs_core_frontend::structs::{
    Comment as CommentData, Thread as ThreadData, User as UserData,
};
use web_sys::RequestInit;
use yew::prelude::*;

enum Msg {
    AddOne,
}

enum Event {
    ThreadLoaded(Thread),
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[derive(PartialEq, Properties, Clone)]
struct CommentProps {
    data: CommentData,
}

#[derive(PartialEq, Properties, Clone)]
struct ThreadProps {
    data: ThreadData,
    comments: Vec<CommentData>,
}

#[derive(PartialEq, Properties, Clone)]
struct AppProps {
    thread_hash: String,
}

#[function_component(Comment)]
fn comment(comment: &CommentProps) -> Html {
    html! {
        <div>
            <h2>{ &comment.data.user_name }</h2>
            <p>{ &comment.data.content }</p>
        </div>
    }
}

#[function_component(Thread)]
fn thread(thread: &ThreadProps) -> Html {
    html! {
        <div>
            <h1>{ &thread.data.name }</h1>
            { for thread.comments.iter().map(|comment|
                html! {
                    <Comment data = {comment.clone()} />
            }) }
        </div>
    }
}

struct App {
    thread: Option<ThreadData>,
    open_comment: Option<CommentData>,
    current_user: Option<UserData>,
    comments: Vec<CommentData>,
}

impl Component for App {
    type Message = Event;
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            thread: None,
            open_comment: None,
            current_user: None,
            comments: Vec::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // ctx.link().send_future();

        html! {
            <div>
                { match &self.thread {
                    Some(thread) => html! {
                        <Thread data={ ThreadData{ hash: "hash".to_string(), name: "test_name".to_string()} }
                        comments={ vec![CommentData{ user_name: "test user".to_string(), content: "comment content".to_string() }] }  />
                            },
                    None => html! {
                        <h1>{ "loading ..." }</h1>
                    }
                } }
            </div>
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct AppContext;

fn main() {
    let props = AppProps {
        thread_hash: "test_hash".to_string(),
    };
    yew::start_app_with_props::<App>(props);
}
