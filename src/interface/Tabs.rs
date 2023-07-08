const DB_PATH: &str = "../database/data.json";

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    name: String,
    category: String,
    description: String,
    age: usize,
    created_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum TabsItem{
    Home,
    Todo,
    Add,
    Delete,
    Quit
}

impl From<TabsItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Todo => 1,
            MenuItem::Add = > 2,
            MenuItem::Delete => 3,
            MenuItem::Quit => 4,
        }
    }
}
