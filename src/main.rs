use std::{ffi::c_void, hash::{BuildHasherDefault, DefaultHasher}, ptr::null_mut};
use acc_shared_memory_rs::{ACCError, ACCSharedMemory};
use itertools::Itertools;

struct InputSimulator {
  init: unsafe extern "C" fn(u32, u32, *mut c_void) -> u32,
  destroy: unsafe extern "C" fn(),
  key_down: unsafe extern "C" fn(u16) -> i32,
  key_up: unsafe extern "C" fn(u16) -> i32,
  _lib: libloading::Library
}

impl InputSimulator {
  fn new() -> Self {
    let lib = unsafe { libloading::Library::new("InputSimulator.dll") }.unwrap();
    let init_sym = unsafe { lib.get(b"IbSendInit") }.unwrap();
    let destroy_sym = unsafe { lib.get(b"IbSendDestroy") }.unwrap();
    let key_down_sym = unsafe { lib.get(b"IbSendKeybdDown") }.unwrap();
    let key_up_sym = unsafe { lib.get(b"IbSendKeybdUp") }.unwrap();

    let init = *init_sym;
    let destroy = *destroy_sym;
    let key_down = *key_down_sym;
    let key_up = *key_up_sym;

    Self {
      _lib: lib,
      init,
      destroy,
      key_down,
      key_up
    }
  }
}

impl Drop for InputSimulator {
  fn drop(&mut self) {
    unsafe { (self.destroy)(); };
  }
}

struct Options {
  mode: OptionsMode
}

enum OptionsMode {
  TC {
    bind: u16,
    min_speed: Option<u32>
  },
  BB {
    bb_inc: u16,
    bb_dec: u16,
    bb_offset: i32,
    bb: u32
  }
}

fn main() {
  let args_str = std::env::args().skip(1).join(" ");
  let mut options_str_tc: Vec<String> = Vec::new();
  let mut options_str_bb: Vec<String> = Vec::new();
  let mut temp_str_tc = String::new();
  let mut temp_str_bb = String::new();
  let mut inside_parens = false;
  let mut inside_brackets = false;

  for c in args_str.chars() {
    match c {
      '(' => {
        inside_parens = true;
        temp_str_tc.clear();
      },
      ')' if inside_parens => {
        options_str_tc.push(temp_str_tc.clone());
        inside_parens = false;
      },
      _ if inside_parens => {
        temp_str_tc.push(c);
      },
      '[' => {
        inside_brackets = true;
        temp_str_bb.clear();
      },
      ']' if inside_brackets => {
        options_str_bb.push(temp_str_bb.clone());
        inside_brackets = false;
      },
      _ if inside_brackets => {
        temp_str_bb.push(c);
      }
      _ => {}
    }
  }

  let mut options: Vec<(u32, Options)> = Vec::new();
  for group in options_str_tc {
    let splitted: Vec<&str> = group.split(" ").collect();

    let perc = splitted[0].parse::<u32>().unwrap();
    let bind = u16::from_str_radix(&splitted[1][2..], 16).unwrap();
    let min_speed = if splitted.len() == 3 { Some(splitted[2].parse::<u32>().unwrap()) } else { None };

    options.push((perc, Options { mode: OptionsMode::TC { bind, min_speed } }));
  }
  let mut bb_binds: (u16, u16, i32) = (0x0, 0x0, 0);
  for (i, group) in options_str_bb.iter().enumerate() {
    let splitted: Vec<&str> = group.split(" ").collect();
    if i == 0 {
      bb_binds.0 = u16::from_str_radix(&splitted[0][2..], 16).unwrap();
      bb_binds.1 = u16::from_str_radix(&splitted[1][2..], 16).unwrap();
      bb_binds.2 = splitted[2].parse::<i32>().unwrap();

      continue;
    }

    let perc = splitted[0].parse::<u32>().unwrap();
    let bb = splitted[1].parse::<f32>().unwrap();
    let bb = (bb * 10f32) as u32;
    let bb = if bb % 2 == 1 { bb + 1 } else { bb };

    options.push((perc, Options { mode: OptionsMode::BB { bb_inc: bb_binds.0, bb_dec: bb_binds.1, bb_offset: bb_binds.2, bb } }));
  }

  let mut options_map: std::collections::HashMap<u32, Vec<Options>, BuildHasherDefault<DefaultHasher>> =
    std::collections::HashMap::with_hasher(BuildHasherDefault::default());
  for (track_perc, option) in options {
    options_map.entry(track_perc).or_insert_with(|| Vec::new()).push(option);
  }

  let mut acc = loop {
    match ACCSharedMemory::new() {
      Ok(mem) => break mem,
      Err(ACCError::SharedMemoryNotAvailable) => {
        println!("Waiting for ACC to start...");
        std::thread::sleep(std::time::Duration::from_secs(1));
      },
      Err(err) => panic!("Unexpected error connecting to ACC: {:?}", err)
    }
  };

  let input_simulator = InputSimulator::new();
  unsafe { (input_simulator.init)(6, 0, null_mut()); };

  let duration_1ms = std::time::Duration::from_millis(1);
  let duration_16ms = std::time::Duration::from_millis(16);
  let mut last_track_perc = 0;
  loop {
    if let Some(data) = acc.read_shared_memory().unwrap() {
      let track_perc = (data.graphics.normalized_car_position * 100.0) as u32;
      if track_perc != last_track_perc {
        if let Some(options) = options_map.get(&track_perc) {
          for option in options {
            match option.mode {
              OptionsMode::TC { bind, min_speed } => {
                let curr_speed = data.physics.speed_kmh as u32;
                if curr_speed > min_speed.unwrap_or(0) {
                  unsafe { (input_simulator.key_down)(bind); };
                  std::thread::sleep(duration_1ms);
                  unsafe { (input_simulator.key_up)(bind); };
                }
              },
              OptionsMode::BB { bb_inc, bb_dec, bb_offset, bb } => {
                let curr_bb = data.physics.brake_bias + (bb_offset as f32 / 100.0);
                let curr_bb = (curr_bb * 1000.0) as u32;
                let curr_bb = if curr_bb % 2 == 1 { curr_bb + 1 } else { curr_bb };
                let bb_diff = curr_bb as i32 - bb as i32;
                let bb_diff = bb_diff / 2;
                let bb_bind = if bb_diff > 0 { bb_dec } else { bb_inc };
                let bb_diff = bb_diff.abs();

                for _ in 0..bb_diff {
                  unsafe { (input_simulator.key_down)(bb_bind); };
                  std::thread::sleep(duration_1ms);
                  unsafe { (input_simulator.key_up)(bb_bind); };
                  std::thread::sleep(duration_1ms);
                }
              }
            }
          }
        }
      }
      last_track_perc = track_perc;
    }
    std::thread::sleep(duration_16ms);
  }
}
