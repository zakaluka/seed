use seed::{prelude::*, *};

mod page;

const ADMIN: &str = "admin";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        ctx: Context {
            logged_user: "John Doe",
        },
        page: Page::new(url),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    page: Page,
}

// ------ Context ------

pub struct Context {
    pub logged_user: &'static str,
}

// ------ Page ------

enum Page {
    Home,
    Admin(page::admin::Model),
    NotFound,
}

impl Page {
    fn new(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(ADMIN) => page::admin::init(url).map_or(Self::NotFound, Self::Admin),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.url()
    }
    pub fn admin_urls(self) -> page::admin::Urls<'a> {
        page::admin::Urls::with_base(self.url().add_path_part(ADMIN))
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::new(url);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    match &model.page {
        Page::Home => div!["Welcome home!"],
        Page::Admin(admin_model) => page::admin::view(admin_model, &model.ctx),
        Page::NotFound => div!["404"],
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
