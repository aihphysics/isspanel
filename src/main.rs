use isspanel::*;

//use clap::{command, Parser};
//use std::thread;
//use std::thread::JoinHandle;
use std::sync::Arc;
use tokio::sync::Notify;

use lightstreamer_client::ls_client::{LightstreamerClient, Transport };
use lightstreamer_client::subscription::{Subscription, SubscriptionMode, Snapshot};
use lightstreamer_client::subscription_listener::SubscriptionListener;
use lightstreamer_client::item_update::ItemUpdate;



struct ISSListener{}

impl SubscriptionListener for ISSListener {
    fn on_item_update(&self, update: &ItemUpdate) {

        let not_available = "N/A".to_string();
        let item_name = update.item_name.clone().unwrap_or(not_available.clone());
        let fields = vec!["TimeStamp", "Value" ];

        let mut output = String::new();

        for field in fields {
            let value = update.get_value(field).unwrap_or(&not_available).clone();
            output.push_str(&format!("{}: {}, ", field, value.to_string()));
        }
        println!("{}, {}", item_name, output);
    }

}


#[tokio::main]
async fn main() {

    // Create a Lightstreamer client
    let mut client = LightstreamerClient::new(
        Some("http://push.lightstreamer.com/lightstreamer"), // Lightstreamer server
        Some("ISSLIVE"),                       // adapter set
        None,                                  // username
        None,                                  // password
    )
    .unwrap();
    client.connection_options.set_forced_transport( Some( Transport::WsStreaming ) );
    client.connection_options.set_slowing_enabled( false );

    let mut subscription = Subscription::new(
        SubscriptionMode::Merge,
        Some(vec!["AIRLOCK000001".to_string() ]),
        Some(vec!["TimeStamp".to_string(), "Value".to_string()]),
        //Some( Snapshot::Yes );
    )
    .unwrap();
    //subscription.set_data_adapter( Some(String::from("ISSLIVE")) ).unwrap();
    subscription.set_requested_snapshot( Some(Snapshot::Yes ) ).unwrap();
    subscription.add_listener( Box::new(ISSListener{} ));
    

    client.subscribe( subscription );
    let notify = Arc::new(Notify::new());
    client.connect(notify).await.unwrap();
        
    println!( "{:?}",client.connection_details );

}
