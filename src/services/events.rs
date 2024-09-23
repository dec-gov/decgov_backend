use std::collections::HashMap;

use anyhow::{anyhow, Result};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

use crate::{
    get_events_by_space,
    types::{event::Event, webhook_event::WebhookEvent},
};

use crate::types::event::{EventData, EventTrigger};

pub async fn trigger_events(
    space_id: u32,
    event_trigger: EventTrigger,
    event_data: HashMap<&str, String>,
) -> Result<()> {
    let events = get_events_by_space(space_id);

    if events.is_none() {
        return Err(anyhow!("Unable to get space by space id"));
    }

    for event in events.unwrap().into_iter() {
        if event.event_trigger != event_trigger {
            continue;
        }

        match event.data {
            EventData::Webhook(data) => handle_webhook_event(data, &event_data).await,
            EventData::Evm(_) => panic!("Evm events are not implemented yet"),
        }
    }

    return Ok(());
}

async fn handle_webhook_event(event: WebhookEvent, event_data: &HashMap<&str, String>) {
    let mut payload = event.payload;

    for (key, value) in event_data.iter() {
        let new_key = format!("${{{}}}", key);
        payload = payload.replace(&new_key, &value);
    }

    let json_utf8: Vec<u8> = payload.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: event.webhook_url,
        method: HttpMethod::POST,
        max_response_bytes: None,
        headers: vec![HttpHeader {
            name: String::from("Content-Type"),
            value: String::from("application/json"),
        }],
        body: request_body,
        transform: None,
    };

    http_request(request, 2_000_000_000).await.unwrap();
}
