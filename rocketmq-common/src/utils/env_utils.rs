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

use crate::common::mix_all::ROCKETMQ_HOME_ENV;

/// Utility functions related to environment variables.
pub struct EnvUtils;

impl EnvUtils {
    /// Gets the value of the specified environment variable.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the environment variable to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing the value of the environment variable, or `None` if the variable is
    /// not set.
    pub fn get_property(key: impl Into<String>) -> Option<String> {
        match std::env::var(key.into()) {
            Ok(value) => Some(value),
            Err(_error) => None,
        }
    }

    /// Gets the value of the ROCKETMQ_HOME environment variable.
    ///
    /// If ROCKETMQ_HOME is not set, it defaults to the current directory and sets ROCKETMQ_HOME
    /// accordingly.
    ///
    /// # Returns
    ///
    /// The value of the ROCKETMQ_HOME environment variable as a `String`.
    pub fn get_rocketmq_home() -> String {
        std::env::var(ROCKETMQ_HOME_ENV).unwrap_or_else(|_| {
            // If ROCKETMQ_HOME is not set, use the current directory as the default value
            let rocketmq_home_dir = std::env::current_dir()
                .unwrap()
                .into_os_string()
                .to_string_lossy()
                .to_string();

            // Set ROCKETMQ_HOME to the current directory
            std::env::set_var(ROCKETMQ_HOME_ENV, rocketmq_home_dir.clone());
            rocketmq_home_dir
        })
    }
}
