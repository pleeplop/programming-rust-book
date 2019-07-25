#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::thread;

#[derive(Copy, Clone, Debug)]
struct City {
    population: i64,
}

impl City {
    fn get_statistic(&self, stat: &Statistic) -> i64 {
        self.population
    }
}

struct Statistic;

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_unstable_by_key(|city| -city.population)
}

fn sort_by_statistic(cites: &mut Vec<City>, stat: &Statistic) {
    cites.sort_unstable_by_key(move |city| -city.get_statistic(stat))
}

fn start_sorting_thread(mut cities: Vec<City>, stat: &'static Statistic) ->
thread::JoinHandle<Vec<City>> {
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(&stat) };
    thread::spawn(move || {
        cities.sort_unstable_by_key(key_fn);
        cities
    })
}

fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
    where F: Fn(&City) -> bool {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn has_monster_attacks(city: &City) -> bool {
    city.population > 1_000
}

fn main() {
    let unsorted_cities = vec![City { population: 3 }, City { population: 1 }, City { population: 2 }];
    let mut cities = unsorted_cities.clone();
    let stat = Statistic {};
    sort_by_statistic(&mut cities, &stat);
    println!("{:?}", cities);
    let n = count_selected_cities(&unsorted_cities, has_monster_attacks);
    let n = count_selected_cities(&unsorted_cities, |city| city.population > 1_000);


    let my_key_fn: fn(&City) -> i64 = |city| city.population;

    let my_str = "hello".to_string();
    let f = || drop(my_str);
//    call_twice(f);

    let dict: HashMap<i32, i32> = HashMap::new();
    let debug_dump_dict = || {
        for (k, v) in &dict {
            println!("{:?}, {:?}", k, v);
        }
    };
    debug_dump_dict();
    debug_dump_dict();

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("Ding! i is now: {}", i);
    };
    call_twice(incr);

    let i = 0;
    let display_next = || {
        println!("Next i value is: {}", i + 1);
    };
    call_twice(display_next);
}

fn call_twice<F>(mut closure: F) where F: FnMut() {
    closure();
    closure();
}

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    fn new() -> BasicRouter {
        BasicRouter { routes: HashMap::new() }
    }

    fn add_route<C>(&mut self, url: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request),
        }
    }
}

fn main_router() {
    let mut router = BasicRouter::new();
    router.add_route("/", get_gcd_response);
    router.add_route("/gcd", get_gcd_form);
}

fn get_gcd_response(request: &Request) -> Response {
    Response { code: 200, headers: HashMap::new(), body: vec![] }
}

fn get_gcd_form(request: &Request) -> Response {
    Response { code: 200, headers: HashMap::new(), body: vec![] }
}

fn not_found_response() -> Response {
    Response { code: 404, headers: HashMap::new(), body: vec![] }
}