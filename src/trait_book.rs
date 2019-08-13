pub mod bank {
    /// Контр агенты
     #[derive(Debug)]
     struct Contragent<'a, T> {
         name: &'a str,
         transactions: Vec<T>
     }

    /// Транзакции
     #[derive(Debug)]
    struct Transactions<'a> {
        id: i32,
        name: &'a str,
    }



    trait MethodsStruct<'a, T>{
        /// Добавление к контр агенту транзакции
        fn add(&mut self,transaction: T);
        /// Редактировании контр агента
        fn update(&mut self,name:&'a str);
    }

    impl <'a,T>MethodsStruct<'a, T> for Contragent<'a, T>{

        fn add(&mut self, transaction: T){
            self.transactions.push(transaction);
        }

        fn update(&mut self, name:&'a str){
            self.name = name;
        }
    }


    pub fn starts() {
        let mut contragents = Contragent {
            name: "Vasya",
            transactions: Vec::new()
        };

        let mut transactions = Transactions {id: 22, name: "pokupka" };

        contragents.add(transactions);

        println!("{:?}",contragents);

        contragents.update("вася");

        println!("{:?}",contragents);

    }
}