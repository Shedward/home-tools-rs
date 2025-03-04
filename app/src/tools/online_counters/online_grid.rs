use chrono::*;
use shared::api::online;

pub type Item = u32;
pub const DAYS_COUNT: usize = 14;
pub const HOURS_COUNT: usize = 24;

#[derive(Debug, Clone)]
pub struct OnlineGrid {
    pub values: [[Item; HOURS_COUNT]; DAYS_COUNT],
    pub max: Item,
}

impl Default for OnlineGrid {
    fn default() -> Self {
        OnlineGrid {
            values: [[0; HOURS_COUNT]; DAYS_COUNT],
            max: 1,
        }
    }
}

impl OnlineGrid {
    pub fn new(online_counters: Vec<online::OnlineCounter>) -> Self {
        let mut grid = OnlineGrid::default();

        let now = Utc::now().date_naive();

        for counter in online_counters {
            let counter_date = counter.start.date_naive();
            let day = (now - counter_date).num_days();
            if day < 0 || day >= DAYS_COUNT as i64 {
                continue;
            }

            let day_index = day as usize;
            let hour_index = counter.start.time().hour() as usize;

            let mut new_value = grid.values[day_index][hour_index];
            new_value += counter.count;

            grid.max = grid.max.max(new_value);

            grid.values[day_index][hour_index] = new_value;
        }

        grid
    }

    pub fn at(&self, day: usize, hour: usize) -> Item {
        self.values[day][hour]
    }

    pub fn rel_at(&self, day: usize, hour: usize) -> f32 {
        let value = self.values[day][hour];
        if value == 0 || self.max == 0 {
            0.0
        } else {
            value as f32 / self.max as f32
        }
    }
}
