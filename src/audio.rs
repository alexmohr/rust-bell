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

use crate::{config, mqtt};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub(crate) struct SoundPlayer {
    config: config::Audio,
}

impl SoundPlayer {
    pub(crate) fn new(audio_config: config::Audio) -> Self {
        SoundPlayer {
            config: audio_config,
        }
    }

    fn play_sound(&self, count: i32) {
        if count <= 0 {
            log::error!("Play count must be > 0");
            return;
        }

        // Get an output stream handle to the default physical sound device.
        // Note that no sound will be played if _stream is dropped
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(output) => output,
            Err(e) => {
                log::error!("Failed to create audio output stream: {}", e);
                return;
            }
        };

        let sink = Sink::try_new(&stream_handle).unwrap();
        for _ in 0..count {
            let file = match File::open(&self.config.sound_file) {
                Ok(f) => BufReader::new(f),
                Err(e) => {
                    log::error!("Failed to open audio file: {}", e);
                    return;
                }
            };
            let source = match Decoder::new(file) {
                Ok(src) => src,
                Err(e) => {
                    log::error!("Failed to decode audio file: {}", e);
                    return;
                }
            };
            sink.append(source);

            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            sink.sleep_until_end();
        }
    }
}

impl mqtt::Callback for SoundPlayer {
    fn on_message(&self, _: &str /* message currently not used */) {
        self.play_sound(self.config.play_count);
    }
}
