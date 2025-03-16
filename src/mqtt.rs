/*
 * Copyright (c) 2025 Alexander Mohr
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use crate::config;


pub(crate) trait Callback {
    fn on_message(&self, message: &str);
}


pub(crate) struct MqttClient {
    config: config::Mqtt,
    //client: Option<AsyncClient>,
    callback: Box<dyn Callback>
}

impl MqttClient {
    // Constructor that initializes the client with host, port, and topic
    pub(crate) fn new(mqtt_config: config::Mqtt, callback: Box<dyn Callback>) -> Self {
        MqttClient {
            config: mqtt_config,
            //client: None,
            callback
        }
    }

    // Connects to the MQTT broker and subscribes to the topic
    pub(crate) fn connect_and_poll(& self) {
        let mut mqtt_options = MqttOptions::new("rust_bell", &self.config.host, self.config.port.unwrap_or(1883));
        if self.config.username.is_some() && self.config.password.is_some() {
            mqtt_options.set_credentials(self.config.username.as_ref().unwrap(), self.config.password.as_ref().unwrap());
        }

        let (mut client, mut connection) = Client::new(mqtt_options, 10);

        client.subscribe(&self.config.topic, QoS::AtMostOnce).unwrap();
        // Iterate to poll the eventloop for connection progress
        for notification in connection.iter().flatten() {
            if let Event::Incoming(Incoming::Publish(packet)) = notification {
                let payload = String::from_utf8(packet.payload.as_ref().to_vec());
                match payload {
                    Ok(payload) => {
                        self.callback.on_message(&payload);
                    }
                    Err(e) => {
                        println!("Error parsing payload: {:?}", e);
                    }
                }
            }
        }
    }
}
