mod pizza;
mod store;

#[cfg(test)]
mod test_super {
    use crate::factory::store::{ChicagoStylePizzaStore, NYStylePizzaStore, PizzaStore};

    #[test]
    fn test_pizza() {
        let ny_store = NYStylePizzaStore::new();
        let chic_store = ChicagoStylePizzaStore::new();

        let pizza = ny_store.order_pizza("cheese");
        println!("Ethan ordered a {}", pizza.get_name());

        let pizza = chic_store.order_pizza("cheese");
        println!("Joel ordered a {}", pizza.get_name());
    }
}
