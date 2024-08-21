/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::Deserialize;
use serde::Serialize;

use crate::protocol::command_custom_header::FromMap;
use crate::rpc::rpc_request_header::RpcRequestHeader;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetConsumerConnectionListRequestHeader {
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,

    #[serde(flatten)]
    pub rpc_request_header: Option<RpcRequestHeader>,
}

impl GetConsumerConnectionListRequestHeader {
    pub const CONSUMER_GROUP: &'static str = "consumerGroup";

    pub fn get_consumer_group(&self) -> &String {
        &self.consumer_group
    }
    pub fn set_consumer_group(&mut self, consumer_group: String) {
        self.consumer_group = consumer_group;
    }
}

impl FromMap for GetConsumerConnectionListRequestHeader {
    type Target = Self;

    fn from(map: &std::collections::HashMap<String, String>) -> Option<Self::Target> {
        Some(GetConsumerConnectionListRequestHeader {
            consumer_group: map.get(Self::CONSUMER_GROUP).cloned().unwrap_or_default(),
            rpc_request_header: <RpcRequestHeader as FromMap>::from(map),
        })
    }
}
