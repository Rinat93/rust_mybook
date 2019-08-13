pub mod Lamdba {
    #[derive(Debug)]
    struct Cities {
        name: String,
        population: i64,
        country: String
    }
    
    // Сортировка на основе замыкающей функции
    fn sort_vec_Cities(cities: &mut Vec<Cities>){
        cities.sort_by_key(|city| -city.population);
    }

    pub fn start() {
        let mut cities = vec![
            Cities {
                name: "Moscow".to_string(),
                population: 100,
                country: "Hai".to_string()
            },Cities {
                name: "Moscow".to_string(),
                population: 321,
                country: "Hai".to_string()
            },Cities {
                name: "Moscow".to_string(),
                population: 1,
                country: "Hai".to_string()
            }
        ];

        sort_vec_Cities(&mut cities);

        println!("{:?}",cities)


    }
}