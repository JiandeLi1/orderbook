use std::{thread, fmt::Pointer};
use ringbuf::RingBuffer;
use core_affinity;
use std::time::{Duration, SystemTime};
use avl::AvlTreeMap;
use std::collections::HashMap;


#[derive(Debug)]
pub enum Side { B, S }

#[derive(Debug)]
pub enum Order_Message {
    N(Order),
    C(Order),
    F
}

#[derive(Debug)]
pub struct  Order{
    pub user: i64,
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



fn main(){
    //A ring buffer is used as storage space for buffering input or output data.
    let rb = RingBuffer::<Order_Message>::new(100);

    //producer from ring buffer is used for adding data to buffer, consumer is used to pop the data from ring buffer
    let (mut prod, mut cons) = rb.split();

    let mut buy_map  = AvlTreeMap::new();
    let mut sell_map = AvlTreeMap::new();
    let mut buy_qtys:HashMap<i64, f64>=HashMap::new();
    let mut sell_qtys:HashMap<i64, f64>=HashMap::new();
    //The working cores from your computer
    let core_ids = core_affinity::get_core_ids().unwrap();
    let core_0=core_ids[0];//core 0
    let core_1=core_ids[1];//core 1
    //let core_2=core_ids[2];//core 2 



    let mut input = vec![ ["N", "1", "IBM", "10", "100", "B", "1" ],
                                   ["N", "1", "IBM", "12", "100", "S", "2" ],
                                   ["N", "2", "IBM", "9", "100", "B", "101" ],
                                   ["N", "2", "IBM", "11", "100", "S", "102" ],
                                   ["N", "1", "IBM", "11", "100", "B", "3" ],
                                   ["N", "2", "IBM", "10", "100", "s", "103" ],
                                   ["N", "1", "IBM", "10", "100", "B", "4" ],
                                   ["N", "2", "IBM", "11", "100", "S", "104" ]];
    // producer
    let pjh = thread::spawn(move || {
        // set only core_0 work for the producer 
        core_affinity::set_for_current(core_0);
        // push some data
        while input.len()>0 {
            let [state, u,s,p,q,side,u_id]= input[0];
            input.drain(0..1);
            let state = state.to_string();
            let order = Order::new(u.parse::<i64>().unwrap(),
                                         s.to_string(), 
                                         p.parse::<i64>().unwrap(),
                                         q.parse::<f64>().unwrap(),
                                         side.to_string(),
                                        u_id.parse::<i64>().unwrap());
           

            match state.as_str() {
                "N"=>{
                    prod.push(Order_Message::N(order));
                },
                "C"=>{
                    prod.push(Order_Message::C(order));
                }
                _=> println!("{}","gg")
            }
        }

        // tell the consumer it can exit
        prod.push(Order_Message::F);
    });
    // consumer
    let cjh = thread::spawn(move || {
        // set only core_1 work for the consumer 
        core_affinity::set_for_current(core_1);

        loop {
            if !cons.is_empty() {  // this is basically using a spin lock
                match cons.pop().unwrap() {
                   Order_Message::N(mut order) => {
                        let p= order.price;
                        if &order.side=="B"  {
                            if !sell_map.get(&p).is_none() {
                                let mut previous_order :&mut Order =sell_map.get_mut(&p).unwrap();
                                if order.qty == previous_order.qty && order.price==previous_order.price{
                                   print_trade( order.user,
                                                  order.userOrderId,
                                                  previous_order.user,
                                                  previous_order.userOrderId,
                                                  order.price,
                                                  order.qty
                                                );
                                    sell_map.remove(&p);
                                    sell_qtys.remove(&p);
                                } else if order.qty > previous_order.qty && order.price==previous_order.price{
                                    order.qty-=previous_order.qty;
                                    print_trade( order.user,
                                                  order.userOrderId,
                                                  previous_order.user,
                                                  previous_order.userOrderId,
                                                  order.price,
                                                  previous_order.qty
                                                );
                                    buy_qtys.insert(p, order.qty);
                                    buy_map.insert(p, order);
                                    sell_map.remove(&p);
                                    sell_qtys.remove(&p);
                                }else if order.qty < previous_order.qty && order.price==previous_order.price{
                                    previous_order.qty-=order.qty;
                                    print_trade( order.user,
                                                  order.userOrderId,
                                                  previous_order.user,
                                                  previous_order.userOrderId,
                                                  order.price,
                                                  previous_order.qty
                                                );
                                    sell_qtys.insert(p,previous_order.qty);
                                }
                            }else{
                                if(buy_qtys.contains_key(&order.price)){
                                    let n=buy_qtys.get(&order.price).unwrap();
                                    buy_qtys.insert(order.price,n+order.qty);
                                }else{
                                    buy_qtys.insert(order.price,order.qty);
                                }
                                print_A(order.user,order.userOrderId);
                                print_B("B", order.price, *buy_qtys.get(&order.price).unwrap());
                                buy_map.insert(p, order);
                            }
                        }else{
                            if !buy_map.get(&p).is_none() {
                                let mut previous_order :&mut Order =buy_map.get_mut(&p).unwrap();
                                if order.qty == previous_order.qty && order.price==previous_order.price{
                                   print_trade( order.user,
                                                  order.userOrderId,
                                                  previous_order.user,
                                                  previous_order.userOrderId,
                                                  order.price,
                                                  order.qty
                                                );
                                    buy_map.remove(&p);
                                    buy_qtys.remove(&p);
                                } else if order.qty >= previous_order.qty && order.price==previous_order.price{
                                    order.qty-=previous_order.qty;
                                    print_trade( order.user,
                                                  order.userOrderId,
                                                  previous_order.user,
                                                  previous_order.userOrderId,
                                                  order.price,
                                                  order.qty
                                                );
                                    sell_qtys.insert(p, order.qty);
                                    sell_map.insert(p, order);
                                    buy_map.remove(&p);
                                    buy_qtys.remove(&p);
                                }else if order.qty < previous_order.qty && order.price==previous_order.price{
                                    previous_order.qty-=order.qty;
                                    print_trade( order.user,
                                                  order.userOrderId,
                                                  previous_order.user,
                                                  previous_order.userOrderId,
                                                  order.price,
                                                  previous_order.qty
                                                );
                                    buy_qtys.insert(p,previous_order.qty);
                                }
                            }else{
                                if(sell_qtys.contains_key(&order.price)){
                                    let n=sell_qtys.get(&order.price).unwrap();
                                    sell_qtys.insert(order.price,n+order.qty);
                                }else{
                                    sell_qtys.insert(order.price,order.qty);
                                }
                                print_A(order.user,order.userOrderId);
                                print_B("S", order.price, *sell_qtys.get(&order.price).unwrap());
                                sell_map.insert(p, order);
                            }
                        }
                   }
                   Order_Message::C(trade_update) => {
                        println!("Cancel order {:?}", trade_update);
                   }
                   Order_Message::F => {
                         break;
                   }
                }
            }
        }
    });
    // wait for threads to complete
    pjh.join().unwrap();
    cjh.join().unwrap();
}

fn print_trade(userBuy:i64,
                userOrderIdBuy:i64,
                userSell:i64,
                userOrderIdSell:i64,
                price:i64,
                qty:f64){
    println!("T userIdbuy:{} UserOrderIdBuy:{} userIdSell:{} UserOrderIdSell:{} price:{} quantity:{}",
                userBuy,
                userOrderIdBuy,
                userSell,
                userOrderIdSell,
                price,
                qty);
}

fn print_A(user:i64, userOrderId:i64){
    println!("A user:{} UserOrderId:{}", user,userOrderId);
}

fn print_B(side:&str, price:i64, qty:f64){
    println!("B side:{} price:{} totalQuantity:{}", side, price, qty);
}