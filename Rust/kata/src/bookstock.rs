use std::collections::HashMap;

pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let stock = match Stock::try_from(list_art) {
        Ok(s) => s,
        Err(StockEvent::EmptyStock) => return String::from(""),
        Err(se) => panic!("Stock input error: {:?}", se),
    };

    let database = stock.get_database().expect("Create database.");
    list_cat
        .iter()
        .map(|&cat| {
            let c = &cat.chars().next().unwrap();
            database.lookup(c).to_string()
        })
        .collect::<Vec<String>>()
        .join(" - ")
}

#[derive(Debug)]
enum ItemError {
    EmptyInput,
    EmptyCode,
    QuantityMissing,
    TooManyFields,
}

#[derive(Debug)]
enum DatabaseError {
    QuantityNotANumber,
}
struct CategorySummary<'a> {
    category: &'a char,
    quantity: i32,
}

impl<'a> ToString for CategorySummary<'a> {
    fn to_string(&self) -> String {
        format!("({} : {})", self.category, self.quantity)
    }
}

struct Database(HashMap<char, i32>);

impl Database {
    fn lookup<'a>(&self, category: &'a char) -> CategorySummary<'a> {
        let quantity = match self.0.get(category) {
            Some(q) => *q,
            None => 0,
        };

        CategorySummary { category, quantity }
    }
}

#[derive(Debug)]
enum StockEvent {
    EmptyStock,
    ItemError(ItemError),
}

struct Stock<'a> {
    items: Vec<Item<'a>>,
}

impl<'a> Stock<'a> {
    fn get_database(&self) -> Result<Database, DatabaseError> {
        let mut database = HashMap::new();

        self.items.iter().try_for_each(|item| {
            // this line is the first copy command
            let quantity = match item.quantity.parse::<i32>() {
                Ok(q) => q,
                Err(_) => return Err(DatabaseError::QuantityNotANumber),
            };
            *database.entry(item.category()).or_insert(0) += quantity;
            Ok(())
        })?;

        Ok(Database(database))
    }
}

impl<'a> TryFrom<Vec<&'a str>> for Stock<'a> {
    type Error = StockEvent;

    fn try_from(items_vec: Vec<&'a str>) -> Result<Self, Self::Error> {
        let items = match items_vec
            .iter()
            .map(|&item| Item::try_from(item))
            .collect::<Result<Vec<Item>, ItemError>>()
        {
            Ok(is) => is,
            Err(ie) => return Err(StockEvent::ItemError(ie)),
        };

        if items.is_empty() {
            return Err(StockEvent::EmptyStock);
        }

        Ok(Stock { items })
    }
}

struct Item<'a> {
    code: &'a str,
    quantity: &'a str,
}

impl<'a> TryFrom<&'a str> for Item<'a> {
    type Error = ItemError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut components = value.split(' ');

        let code = match components.next() {
            Some(c) => c,
            None => return Err(ItemError::EmptyInput),
        };

        let quantity_str = match components.next() {
            Some(q) => q,
            None => return Err(ItemError::QuantityMissing),
        };

        if components.next().is_some() {
            return Err(ItemError::TooManyFields);
        }

        Item::create(code, quantity_str)
    }
}

impl<'a> Item<'a> {
    /// Creates an Item with zero-copy.
    fn create(code: &'a str, quantity: &'a str) -> Result<Item<'a>, ItemError> {
        if code.is_empty() {
            return Err(ItemError::EmptyCode);
        }

        Ok(Item { code, quantity })
    }

    fn category(&self) -> char {
        self.code.chars().next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("-;");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

        b = vec![];
        c = vec!["A", "B"];
        dotest(b, c, "");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec![];
        dotest(b, c, "");
    }
}
