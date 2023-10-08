mod constants;
mod notifier;

use crate::constants::{
    LINK, MAX_NUMBER_OF_TRIES, SECS_BETWEEN_REQUESTS, SECS_TO_SLEEP_AFTER_REQUEST_ERROR, SELECTOR,
};
use crate::notifier::TelegramNotifier;
use scraper::{Html, Selector};
use std::thread::sleep;
use std::time::Duration;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

type NumberOfAds = usize;

fn main() {
    dotenv::dotenv().unwrap();

    let log_file = rolling::daily("./logs", "app");
    tracing_subscriber::fmt()
        .json()
        .with_writer(log_file.and(std::io::stdout))
        .init();

    tracing::info!("starting app");

    let tg_notifier = TelegramNotifier::new(
        std::env::var("TG_BOT_TOKEN").unwrap(),
        std::env::var("TG_CHAT_ID").unwrap(),
    );

    tg_notifier.send_message("test message").unwrap();

    let time_between_requests = Duration::from_secs(SECS_BETWEEN_REQUESTS);
    let mut number_of_ads = get_number_of_ads(&tg_notifier);
    sleep(time_between_requests);

    loop {
        let new_number_of_ads = get_number_of_ads(&tg_notifier);

        if new_number_of_ads > number_of_ads {
            tracing::info!("new ads appeared");
            tg_notifier
                .send_message("New ads appeared!")
                .expect("failed to send tg message");
            number_of_ads = new_number_of_ads;
        } else {
            number_of_ads = new_number_of_ads;
        }

        sleep(time_between_requests);
    }
}

fn get_number_of_ads(notifier: &TelegramNotifier) -> NumberOfAds {
    tracing::info!("requesting lun");

    let mut number_of_tries = MAX_NUMBER_OF_TRIES;

    let response;
    loop {
        let result = reqwest::blocking::get(LINK).map_err(|err| {
            tracing::error!("failed to request lun: {:?}", err);
            err
        });

        match result {
            Ok(resp) => {
                response = resp;
                break;
            }
            Err(err) => {
                number_of_tries -= 1;
                tracing::info!("number of tries to request lun left: {}", number_of_tries);
                if number_of_tries == 0 {
                    tracing::info!("number of tries to request lun exceeded");
                    notifier
                        .send_message("number of tries to request lun exceeded")
                        .unwrap();
                    panic!("failed to request lun: {:?}", err);
                }
                sleep(Duration::from_secs(SECS_TO_SLEEP_AFTER_REQUEST_ERROR));
            }
        }
    }

    let html_content = response.text().unwrap();

    let document = Html::parse_document(&html_content);

    // subtract 2 items as they're not ads
    let number_of_ads = document.select(&Selector::parse(SELECTOR).unwrap()).count() - 2;
    tracing::info!("successfully retrieved {} ads", number_of_ads);

    number_of_ads
}
