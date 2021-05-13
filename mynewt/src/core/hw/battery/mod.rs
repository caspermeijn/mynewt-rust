/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2020 Casper Meijn <casper@meijn.net>
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

use core::time::Duration;

pub struct Battery {
    battery: &'static mut mynewt_sys::battery,
}

pub enum PropertyType {
    None = mynewt_sys::battery_property_type_t_BATTERY_PROP_NONE as isize,
    /* Battery status supported */
    Status = mynewt_sys::battery_property_type_t_BATTERY_PROP_STATUS as isize,
    /* Battery capacity level supported */
    CapacityLevel = mynewt_sys::battery_property_type_t_BATTERY_PROP_CAPACITY_LEVEL as isize,
    /* Battery capacity in mAh */
    Capacity = mynewt_sys::battery_property_type_t_BATTERY_PROP_CAPACITY as isize,
    /* Predicted full battery capacity in mAh */
    CapacityFull = mynewt_sys::battery_property_type_t_BATTERY_PROP_CAPACITY_FULL as isize,
    /* Current battery temperature in C */
    TempNow = mynewt_sys::battery_property_type_t_BATTERY_PROP_TEMP_NOW as isize,
    /* Ambient temperature in C */
    TempAmbient = mynewt_sys::battery_property_type_t_BATTERY_PROP_TEMP_AMBIENT as isize,
    /* Minimum voltage in V */
    VoltageMin = mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_MIN as isize,
    /* Maximum voltage in V */
    VoltageMax = mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_MAX as isize,
    /* Minimum designed voltage in V */
    VoltageMinDesign = mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_MIN_DESIGN as isize,
    /* Maximum designed voltage in V */
    VoltageMaxDesign = mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_MAX_DESIGN as isize,
    /* Current voltage in V */
    VoltageNow = mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_NOW as isize,
    /* Current average voltage in V */
    VoltageAverage = mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_AVG as isize,
    /* Maximum current in mAh */
    CurrentMax = mynewt_sys::battery_property_type_t_BATTERY_PROP_CURRENT_MAX as isize,
    /* Current level at this time in mAh */
    CurrentNow = mynewt_sys::battery_property_type_t_BATTERY_PROP_CURRENT_NOW as isize,
    /* Average current level in mAh */
    CurrentAverage = mynewt_sys::battery_property_type_t_BATTERY_PROP_CURRENT_AVG as isize,
    /* State-Of-Charge, current capacity 0-100 % */
    StateOfCharge = mynewt_sys::battery_property_type_t_BATTERY_PROP_SOC as isize,
    /* State-Of-Health, current battery state of health 0-100 % */
    StateOfHealth = mynewt_sys::battery_property_type_t_BATTERY_PROP_SOH as isize,
    /* Predicted time to complete discharge in seconds */
    TimeToEmptyNow = mynewt_sys::battery_property_type_t_BATTERY_PROP_TIME_TO_EMPTY_NOW as isize,
    /* Predicted time to full capacity when charging in seconds */
    TimeToFullNow = mynewt_sys::battery_property_type_t_BATTERY_PROP_TIME_TO_FULL_NOW as isize,
    /* Number of full discharge/charge cycles */
    CycleCount = mynewt_sys::battery_property_type_t_BATTERY_PROP_CYCLE_COUNT as isize,
}

pub enum BatteryStatus {
    Unknown = mynewt_sys::battery_status_t_BATTERY_STATUS_UNKNOWN as isize,
    /// Charger connected, battery charging */
    Charging = mynewt_sys::battery_status_t_BATTERY_STATUS_CHARGING as isize,
    /// Charger not connected, battery discharging */
    Discharging = mynewt_sys::battery_status_t_BATTERY_STATUS_DISCHARGING as isize,
    /// Charger connected, not charging */
    NotCharging = mynewt_sys::battery_status_t_BATTERY_STATUS_NOT_CHARGING as isize,
    /// Charger connected, not charging - battery full
    Full = mynewt_sys::battery_status_t_BATTERY_STATUS_FULL as isize,
}

impl From<mynewt_sys::battery_status_t> for BatteryStatus {
    fn from(status: mynewt_sys::battery_status_t) -> Self {
        match status {
            mynewt_sys::battery_status_t_BATTERY_STATUS_UNKNOWN => BatteryStatus::Unknown,
            mynewt_sys::battery_status_t_BATTERY_STATUS_CHARGING => BatteryStatus::Charging,
            mynewt_sys::battery_status_t_BATTERY_STATUS_DISCHARGING => BatteryStatus::Discharging,
            mynewt_sys::battery_status_t_BATTERY_STATUS_NOT_CHARGING => BatteryStatus::NotCharging,
            mynewt_sys::battery_status_t_BATTERY_STATUS_FULL => BatteryStatus::Full,
            _ => panic!(),
        }
    }
}

pub enum CapacityLevel {
    Unknown = mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_UNKNOWN as isize,
    Critical = mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_CRITICAL as isize,
    Low = mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_LOW as isize,
    Normal = mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_NORMAL as isize,
    High = mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_HIGH as isize,
    Full = mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_FULL as isize,
}

impl From<mynewt_sys::battery_capacity_level_t> for CapacityLevel {
    fn from(capacity_level: mynewt_sys::battery_capacity_level_t) -> Self {
        match capacity_level {
            mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_UNKNOWN => {
                CapacityLevel::Unknown
            }
            mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_CRITICAL => {
                CapacityLevel::Critical
            }
            mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_LOW => CapacityLevel::Low,
            mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_NORMAL => {
                CapacityLevel::Normal
            }
            mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_HIGH => CapacityLevel::High,
            mynewt_sys::battery_capacity_level_t_BATTERY_CAPACITY_LEVEL_FULL => CapacityLevel::Full,
            _ => panic!(),
        }
    }
}

pub enum PropertyValue {
    None,
    /// in milli Volt
    Voltage(i32),
    /// in milli Ampère
    Current(i32),
    /// in milli Ampère hour
    Capacity(u32),
    /// SOC in percent (0%..100%)
    StateOfCharge(u8),
    /// SOH in percent (0%..100%)
    StateOfHealth(u8),
    /// Temperature in degrees Celsius
    Temperature(f32),
    /// Time in seconds
    Time(u32),
    /// Number of charge cycles
    CycleCount(u16),
    Status(BatteryStatus),
    CapacityLevel(CapacityLevel),
}

pub struct Property {
    property: *mut mynewt_sys::battery_property,
}

impl Battery {
    pub fn get_by_number(num: usize) -> Result<Battery, ()> {
        let os_dev = unsafe { mynewt_sys::battery_get_battery(num as i32) };
        if os_dev != core::ptr::null_mut() {
            Ok(Battery {
                battery: unsafe { (os_dev as *mut mynewt_sys::battery).as_mut() }.unwrap(),
            })
        } else {
            Err(())
        }
    }

    pub fn open(dev_name: &str) -> Result<Battery, ()> {
        assert!(dev_name.ends_with('\0'));
        let os_dev =
            unsafe { mynewt_sys::os_dev_open(dev_name.as_ptr(), 0, core::ptr::null_mut()) };
        if os_dev != core::ptr::null_mut() {
            Ok(Battery {
                battery: unsafe { (os_dev as *mut mynewt_sys::battery).as_mut() }.unwrap(),
            })
        } else {
            Err(())
        }
    }

    pub fn find_property(&mut self, property_type: PropertyType) -> Result<Property, ()> {
        let property = unsafe {
            mynewt_sys::battery_find_property(
                &mut self.battery.b_dev,
                property_type as u8,
                mynewt_sys::battery_property_flags_t_BATTERY_PROPERTY_FLAGS_NONE,
                core::ptr::null_mut(),
            )
        };
        if property != core::ptr::null_mut() {
            Ok(Property { property })
        } else {
            Err(())
        }
    }

    pub fn set_poll_rate(&mut self, poll_rate: Duration) -> Result<(), i32> {
        let rc = unsafe {
            mynewt_sys::battery_set_poll_rate_ms(
                &mut self.battery.b_dev,
                poll_rate.as_millis() as u32,
            )
        };
        if rc == 0 {
            Ok(())
        } else {
            Err(rc)
        }
    }
}

impl Property {
    pub fn change_subscribe(&self) {
        unimplemented!()
    }

    pub fn is_valid(&self) -> bool {
        let property = unsafe { *self.property };
        property.bp_valid() != 0
    }

    pub fn get_type(&self) -> PropertyType {
        let property = unsafe { *self.property };
        match property.bp_type {
            mynewt_sys::battery_property_type_t_BATTERY_PROP_STATUS => PropertyType::Status,
            mynewt_sys::battery_property_type_t_BATTERY_PROP_CAPACITY_LEVEL => {
                PropertyType::CapacityLevel
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_CAPACITY => PropertyType::Capacity,
            mynewt_sys::battery_property_type_t_BATTERY_PROP_CAPACITY_FULL => {
                PropertyType::CapacityFull
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_TEMP_NOW => PropertyType::TempNow,
            mynewt_sys::battery_property_type_t_BATTERY_PROP_TEMP_AMBIENT => {
                PropertyType::TempAmbient
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_MIN => {
                PropertyType::VoltageMin
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_MAX => {
                PropertyType::VoltageMax
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_MIN_DESIGN => {
                PropertyType::VoltageMinDesign
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_MAX_DESIGN => {
                PropertyType::VoltageMaxDesign
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_NOW => {
                PropertyType::VoltageNow
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_VOLTAGE_AVG => {
                PropertyType::VoltageNow
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_CURRENT_MAX => {
                PropertyType::CurrentMax
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_CURRENT_NOW => {
                PropertyType::CurrentNow
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_CURRENT_AVG => {
                PropertyType::CurrentAverage
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_SOC => PropertyType::StateOfCharge,
            mynewt_sys::battery_property_type_t_BATTERY_PROP_SOH => PropertyType::StateOfHealth,
            mynewt_sys::battery_property_type_t_BATTERY_PROP_TIME_TO_EMPTY_NOW => {
                PropertyType::TimeToEmptyNow
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_TIME_TO_FULL_NOW => {
                PropertyType::TimeToFullNow
            }
            mynewt_sys::battery_property_type_t_BATTERY_PROP_CYCLE_COUNT => {
                PropertyType::CycleCount
            }
            _ => PropertyType::None,
        }
    }

    pub fn get_value(&self) -> Option<PropertyValue> {
        let property = unsafe { *self.property };
        if self.is_valid() {
            Some(match self.get_type() {
                PropertyType::None => PropertyValue::None,
                PropertyType::Status => PropertyValue::Status(BatteryStatus::from(unsafe {
                    property.bp_value.bpv_status
                })),
                PropertyType::CapacityLevel => {
                    PropertyValue::CapacityLevel(CapacityLevel::from(unsafe {
                        property.bp_value.bpv_capacity_level
                    }))
                }
                PropertyType::Capacity => {
                    PropertyValue::Capacity(unsafe { property.bp_value.bpv_capacity })
                }
                PropertyType::CapacityFull => {
                    PropertyValue::Capacity(unsafe { property.bp_value.bpv_capacity })
                }
                PropertyType::TempNow => {
                    PropertyValue::Temperature(unsafe { property.bp_value.bpv_temperature })
                }
                PropertyType::TempAmbient => {
                    PropertyValue::Temperature(unsafe { property.bp_value.bpv_temperature })
                }
                PropertyType::VoltageMin => {
                    PropertyValue::Voltage(unsafe { property.bp_value.bpv_voltage })
                }
                PropertyType::VoltageMax => {
                    PropertyValue::Voltage(unsafe { property.bp_value.bpv_voltage })
                }
                PropertyType::VoltageMinDesign => {
                    PropertyValue::Voltage(unsafe { property.bp_value.bpv_voltage })
                }
                PropertyType::VoltageMaxDesign => {
                    PropertyValue::Voltage(unsafe { property.bp_value.bpv_voltage })
                }
                PropertyType::VoltageNow => {
                    PropertyValue::Voltage(unsafe { property.bp_value.bpv_voltage })
                }
                PropertyType::VoltageAverage => {
                    PropertyValue::Voltage(unsafe { property.bp_value.bpv_voltage })
                }
                PropertyType::CurrentMax => {
                    PropertyValue::Current(unsafe { property.bp_value.bpv_current })
                }
                PropertyType::CurrentNow => {
                    PropertyValue::Current(unsafe { property.bp_value.bpv_current })
                }
                PropertyType::CurrentAverage => {
                    PropertyValue::Current(unsafe { property.bp_value.bpv_current })
                }
                PropertyType::StateOfCharge => {
                    PropertyValue::StateOfCharge(unsafe { property.bp_value.bpv_soc })
                }
                PropertyType::StateOfHealth => {
                    PropertyValue::StateOfHealth(unsafe { property.bp_value.bpv_soh })
                }
                PropertyType::TimeToEmptyNow => {
                    PropertyValue::Time(unsafe { property.bp_value.bpv_time_in_s })
                }
                PropertyType::TimeToFullNow => {
                    PropertyValue::Time(unsafe { property.bp_value.bpv_time_in_s })
                }
                PropertyType::CycleCount => {
                    PropertyValue::CycleCount(unsafe { property.bp_value.bpv_cycle_count })
                }
            })
        } else {
            None
        }
    }
}
