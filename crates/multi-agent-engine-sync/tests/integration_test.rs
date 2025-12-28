/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use multi_agent_engine_sync::message::{
    channel, direction, MessageChannel, MessageReceiver, MessageSender,
};

enum Message {}

#[test]
fn test_integration_message_channel() {
    let channel_fc_ts: channel::FromControllerToSystem<Message> = MessageChannel::unbounded();
    let _: (
        MessageSender<Message, direction::ToSystem>,
        MessageReceiver<Message, direction::FromController>,
    ) = channel_fc_ts.split();

    let channel_fs_tc: channel::FromSystemToController<Message> = MessageChannel::bounded(0);
    let _: (
        MessageSender<Message, direction::ToController>,
        MessageReceiver<Message, direction::FromSystem>,
    ) = channel_fs_tc.split();

    let channel_fc_tc: channel::FromControllerToController<Message> = MessageChannel::bounded(10);
    let _: (
        MessageSender<Message, direction::ToController>,
        MessageReceiver<Message, direction::FromController>,
    ) = channel_fc_tc.split();
}
