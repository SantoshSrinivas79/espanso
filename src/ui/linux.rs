/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::process::Command;
use super::MenuItem;
use log::error;

pub struct LinuxUIManager {}

impl super::UIManager for LinuxUIManager {
    fn notify(&self, message: &str) {
        let res = Command::new("notify-send")
                        .args(&["-t", "2000", "espanso", message])
                        .output();

        if let Err(e) = res {
            error!("Could not send a notification, error: {}", e);
        }
    }

    fn show_menu(&self, _menu: Vec<MenuItem>) {
        // Not implemented on linux
    }

    fn cleanup(&self) {
        // Nothing to do here
    }
}

impl LinuxUIManager {
    pub fn new() -> LinuxUIManager {
        LinuxUIManager{}
    }
}