pub mod server {
    type BoxedCallback = Box<Fn(&Request) -> Response>;

    // То что принимаем
    struct Request {
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: Vec<u8>
    }

    // То что отдаем
    struct Response {
        code: u32,
        headers: HashMap<String, String>,
        body: Vec<u8>
    }

    // Роутер
    struct Router {
        routes: HashMap<String, BoxedCallback>
    } 

    impl Router{
        // Инициализируем роутер
        fn new() -> Router {
            Router { routes: HashMap::new()}
        }

        // Добавляем маршрут
        fn add<C>(&mut self, url: &str, callback: C) where C: Fn(&Request) -> Response+'static {
            self.routes.insert(url.to_string(), Box::new(callback))
        }

        fn hande_request(&self, request: &Request) -> Response {
            match self.routes.get(&request.url) {
                Some(callback) => callback(request),
                None => println!("Not found"),
            }
        }
    }
}