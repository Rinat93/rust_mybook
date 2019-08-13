
/// Описание работы итераторов и адаптеров
pub mod Iters {
    use std::str::FromStr;
    use std::collections::HashMap;
    use std::option::Option;

    fn triangle(n: i32) -> i32 {
        (1..n+1).fold(0, |sum,item| sum + item)
    }

    // Взять из строки числа
    fn str_Int_raz() {
        let text2 = "  pointer .25 \n giraffes\n \nsquid .3".to_string();
        for i in text2.split_whitespace().filter_map(|w| f64::from_str(w).ok()) {
            println!("{:?}",i.sqrt());
        }
    }

    // flat map
    fn flats_maps(){
        let mut cities = HashMap::new();
        cities.insert("Japan", vec!["Tokio", "Kioto"]);
        cities.insert("Russia", vec!["Moscow", "Tyumen"]);
        cities.insert("Austria", vec!["Vena", "Brandenburg"]);
        cities.insert("USA", vec!["Vachington", "New yourk"]);
        let country = ["Japan", "Russia", "Austria", "USA"];

        for &city in country.iter().flat_map(|country| &cities[country]) {
            println!("{}",city)
        }
        let mut it = country.iter().flat_map(|country| &cities[country]);
        println!("{:?}",it.next());
    }

    // adapters scan
    fn scans_adapters() {
        let iters = {0..10}.scan(0,|sum,item| {
            *sum += item;
            // Завершить если более 10
            if *sum > 10 {
                None
            } else {
                Some(item * item)
            }
        });
        println!("{:?}",iters.collect::<Vec<i32>>());
    }

    pub fn start() {
        let tr = triangle(10);
        println!("{:?}",tr);
        let text = "  pointer \n giraffes\n \nsquid".to_string();
        let r: Vec<&str>=text.lines().map(str::trim).filter(|s| *s != "pointer" ).collect();
        println!("{:?}",r);
        str_Int_raz();
        flats_maps();
        scans_adapters();
    }

}