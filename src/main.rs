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

mod audio;
mod cli;
mod config;
mod mqtt;

fn main() {
    let args = cli::parse_arguments();
    let config = match config::read_config(args.config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to parse config: {}", e);
            std::process::exit(1);
        }
    };

    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or(config.general.log_level.unwrap_or("error".to_string())),
    )
    .init();

    let sound_player = Box::new(audio::SoundPlayer::new(config.audio));
    let client = mqtt::MqttClient::new(config.mqtt, sound_player);
    client.connect_and_poll();
}
