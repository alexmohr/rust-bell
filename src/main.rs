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

mod cli;
mod config;
mod mqtt;

struct SoundPlayer;
impl mqtt::Callback for SoundPlayer {
    fn on_message(&self, message: &str) {
        println!("Received message: {}", message);
    }
}

//#[tokio::main]
fn main() {
    let args = cli::parse_arguments();
    let config = match config::read_config(args.config) {
        Ok(config) => {
            let sound_player: Box<SoundPlayer> = Box::new(SoundPlayer);
            let client = mqtt::MqttClient::new(
                config.mqtt,
                 sound_player,
            );
            println!("Starting mqtt server...");
            client.connect_and_poll();
      }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
