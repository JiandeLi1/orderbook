# orderbook
## Using rust to create a high frequency trading order book

The Progarm is OOP, I make the class call Order, and two enums call Side and Order_Message
```
Order{user: i64,
    pub symbol: String,
    pub price: i64,
    pub qty: f64,
    pub side : String,
    pub userOrderId: i64
}

impl Order{
    fn new (user: i64, symbol:String, price: i64, qty: f64, side:String, userOrderId: i64)->Order{
        Order{
            user,
            symbol,
            price,
            qty,
            side,
            userOrderId
        }
    }
}

pub enum Side { B, S }//Both side Bid and sell

pub enum Order_Message {
    N(Order),//New order
    C(Order),//Cancel order
    F//flush
}
``` 
<br />

## How to run?
cargo run <br/>

## crate
ringbuf = "0.2.7" <br />
core_affinity = "0.5.10" <br />
avl = "0.6.2" <br />

In high frequency trading, to ensure that every trade is not missed, we need a buffer, so I used ring <br />
buffer in the progarm. In order to ensure that a core only does one thing (in order to improve <br />
performance), the core_affinity library is used to make the taking order for one core and processing order <br />
for the other core. I use the avl tree map to store the state of the order (KEY: price, VALUE: order). <br />
Because of in order to implement the Limit Order Book, I need to sort the key (price) of the map.

## Time and space complexities
For time complexity, To implement the Limit Order Book, we need to traverse the keys(prices) in the map,<br /> 
there is O(N). To sort the avl tree map, time complexity is O(logN). Although mapping data is O(1),If there <br />
 are N order, we need to do N time traverse map and sort the map, the time complexity is not good, that is <br />
 O(N^2*logN). <Br />
 However, I didn't finish the Limit Order Book function, I just finish the exact matching to revent rejection<br /> 
 of orders, so now, my progarm time complexity is O(NlogN).
 
 <br />
 
 Need to sort orders and total ptys by map, space complexity is O(N).