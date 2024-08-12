use isspanel::*;

//use clap::{command, Parser};
use std::{thread, time};
//use std::thread::JoinHandle;

// Server imports
//
use lightstreamer_client::{
    ls_client::{LightstreamerClient, Transport},
    client_listener::ClientListener,
    subscription::{Snapshot, Subscription, SubscriptionMode},
};

use std::sync::Arc;
use tokio::sync::{watch, Barrier, Notify, futures, Mutex};
//use lightstreamer_client::subscription_listener::SubscriptionListener;
//use lightstreamer_client::item_update::ItemUpdate;

// ratatui imports
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};
use std::io::{stdout, Result};

//fn initialize_logging() {
//    let collector = tracing_subscriber::fmt()
//        .with_level(false)
//        .with_target(false)
//        .with_thread_ids(false)
//        .with_thread_names(false)
//        .with_target(false)
//        .with_ansi(true)
//        .pretty()
//        .finish();
//    tracing::subscriber::set_global_default(collector).unwrap();
//}


//#[derive(Debug)]
//struct BlankListener{}
//
//impl ClientListener for BlankListener{
//    fn on_listen_end(&self) { }
//    fn on_listen_start(&self) {}
//    fn on_property_change(&self, _property: &str){ }
//    fn on_server_error(&self, _code: i32, _message: &str){ }
//    fn on_status_change(&self, _status: &str){ }
//}

//async fn start_server( shutdown: Arc<Notify> ) -> ( &LightstreamerClient, impl Future<Output = std::result::Result<(), Box<dyn std::error::Error>>>) {
fn get_client() -> Arc<Mutex<LightstreamerClient>> {//, impl Future<Output = std::result::Result<(), Box<dyn std::error::Error>>>) //{
  let mut client = LightstreamerClient::new(
        Some("http://push.lightstreamer.com/lightstreamer"),
        Some("ISSLIVE"),
        None,
        None,
    )
    .unwrap();
    client
        .connection_options
        .set_forced_transport(Some(Transport::WsStreaming));
    client.connection_options.set_slowing_enabled(false);
    client.connection_options.set_idle_timeout(1000).unwrap();
    println!( "{:?}", client.get_listeners() );
    client.add_listener( Box::new(BlankListener{}) );

    let mut subscription = Subscription::new(
        SubscriptionMode::Merge,
        Some(
            ISS_TELEMS
                .iter()
                .map(|&s| String::from(s))
                .collect::<Vec<String>>(),
        ),
        Some(vec!["TimeStamp".to_string(), "Value".to_string()]),
    )
    .unwrap();

    subscription
        .set_requested_snapshot(Some(Snapshot::Yes))
        .unwrap();
    //subscription.add_listener(Box::new(ISSListener::new(tx) ));
    subscription.add_listener(Box::new(ISSListener {}));

    client.subscribe(subscription);
    Arc::new( Mutex::new(client) )
    //( &client, client.connect(shutdown  ) )
}

#[tokio::main]
async fn main() -> Result<()> {
    
    let close= Arc::new(Notify::new());
    let mut client= get_client();
    let cli = client.clone();

    tokio::task::spawn( async move {
      //let gag = Gag::stdout().unwrap(); 
      let mut cli = cli.lock().await;
      cli.connect( close ).await.unwrap();
    });

    thread::sleep( time::Duration::from_millis(100));

    // ratatui testing
    
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    loop {
        terminal
            .draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .white()
                        .on_blue(),
                    area,
                );
            })
            .unwrap();

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }


    terminal.draw(|frame| {
        let area = frame.size();
        frame.render_widget(Paragraph::new("Exiting").white().on_red(), area);
    })?;

    thread::sleep( time::Duration::from_millis(2000));
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
//.instrument(tracing::info_span!("connect"))
