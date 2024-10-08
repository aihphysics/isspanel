use isspanel::*;

//use clap::{command, Parser};
use std::{thread, time};
//use std::thread::JoinHandle;

// Server imports
//
use lightstreamer_client::{
    client_listener::ClientListener,
    ls_client::{LightstreamerClient, LogType, Transport},
    subscription::{Snapshot, Subscription, SubscriptionMode},
};

use std::sync::Arc;
use tokio::sync::{futures, watch, Barrier, Mutex, Notify};
use tracing_subscriber;

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

fn initialize_logging() {
    let collector = tracing_subscriber::fmt()
        .with_level(false)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_target(false)
        .with_ansi(true)
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(collector).unwrap();
}

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

use tokio::sync::mpsc::{ Sender, Receiver };

fn get_client() -> (Arc<Mutex<LightstreamerClient>>, Receiver<f32> ) {
    //, impl Future<Output = std::result::Result<(), Box<dyn std::error::Error>>>) //{
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
    client.set_logging_type(LogType::TracingLogs);

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


    let (tx, rx) =  tokio::sync::mpsc::channel::<f32>(1000);
    subscription.add_listener(Box::new(ISSListener::new(tx) ));
    //subscription.add_listener(Box::new(ISSListener::<f32>{}));

    client.subscribe(subscription);
    ( Arc::new(Mutex::new(client)), rx )
}

#[tokio::main]
async fn main() -> Result<()> {
    let close = Arc::new(Notify::new());
    let ( mut client, mut rx ) = get_client();

    let cli = client.clone();
    tokio::task::spawn(async move {
        let mut cli = cli.lock().await;
        cli.connect(close).await.unwrap();
    });

    for _ in 0..1000 {
      let val = rx.recv().await.unwrap();
      println!( "recieved: {}", val );
      thread::sleep(time::Duration::from_millis(100));
    }

    Ok(())
}

    // ratatui testing

    //stdout().execute(EnterAlternateScreen)?;
    //enable_raw_mode()?;
    //let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    //terminal.clear()?;
    //loop {
    //    terminal
    //        .draw(|frame| {
    //            let area = frame.size();
    //            frame.render_widget(
    //                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
    //                    .white()
    //                    .on_blue(),
    //                area,
    //            );
    //        })
    //        .unwrap();

    //    if event::poll(std::time::Duration::from_millis(16))? {
    //        if let event::Event::Key(key) = event::read()? {
    //            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
    //                break;
    //            }
    //        }
    //    }
    //}

    //terminal.draw(|frame| {
    //    let area = frame.size();
    //    frame.render_widget(Paragraph::new("Exiting").white().on_red(), area);
    //})?;

    //thread::sleep( time::Duration::from_millis(2000));
    //stdout().execute(LeaveAlternateScreen)?;
    //disable_raw_mode()?;

//.instrument(tracing::info_span!("connect"))
