mod pizza_order{
  // pub(in path) makes an item visible within the provided path . path must be a parent module of the item whose visibility is being declared. pub(crate) makes an item visible within the current crate.
  pub struct Pizza{
    pub dough: String,
    pub cheese: String,
    pub topping: String,
  }
  impl Pizza{
    pub fn lunch(topping: &str) ->Pizza{
      Pizza{
        dough: String::from("regular dough"),
        cheese: String::from("Mozzarella"),
        // user defined
        topping: String::from(topping),
      }
    }
  }
  pub mod help_customer{
    fn seat_at_table(){
      println!("Customer seated at table ");
    }
    // making parents public does not make children public
    pub fn take_order(){
      seat_at_table();
      // super gonna allow me to access in the parents scope
      let cust_pizza: super::Pizza =
        super::Pizza::lunch("veggies");
      server_customer(cust_pizza);
    }
    fn server_customer(cust_pizza:super::Pizza){
      println!("the customer is server a regular pizza with {}", cust_pizza.topping);
    }
  }
}

pub fn order_food(){
  crate::modules::pizza_order::help_customer::take_order()
}